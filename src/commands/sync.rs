use crate::infra::projects;
use crate::view::confirm;

use anyhow::Result;
use regex::{Captures, Regex};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct BranchInfo {
    current: bool,
    name: String,
    remote: Option<String>,
    gone: bool,
    behind: bool,
    ahead: bool,
}

impl From<Captures<'_>> for BranchInfo {
    fn from(captures: Captures) -> Self {
        Self {
            current: captures.name("atual").is_some(),
            name: captures["local"].to_string(),
            remote: captures.name("remoto")
                .map(|m| m.as_str().to_string()),
            gone: captures.name("status")
                .filter(|m| m.as_str().contains("gone"))
                .is_some(),
            behind: captures.name("status")
                .filter(|m| m.as_str().contains("behind"))
                .is_some(),
            ahead: captures.name("status")
                .filter(|m| m.as_str().contains("ahead"))
                .is_some(),
        }
    }
}

pub fn run(project: Option<&str>) -> Result<()> {
    let project_dir = project.and_then(projects::dir);
    let branches_info = branches_info(project_dir.as_deref());
    remove_deleted_branches(&branches_info, project_dir.as_deref());
    update_behind_branches(&branches_info, project_dir.as_deref());
    update_divergent_branches(&branches_info, project_dir.as_deref());
    Ok(())
}

fn branches_info(project_dir: Option<&Path>) -> Vec<BranchInfo> {
    println!("Buscando atualizações no remoto");
    create_git_command(project_dir)
        .args(["fetch", "--prune", "--all"])
        .output()
        .expect("Não foi possível atualizar a branch local");
    let output = create_git_command(project_dir)
        .args(["branch", "-vv"])
        .output()
        .expect("Não foi possível identificar");
    if output.status.success() {
        let regex = Regex::new(r"^(?P<atual>\*)?\s+(?P<local>[^\s]+)\s+(?P<hash>[0-9a-f]{7,})\s+(?:\[(?P<remoto>[^\]:]+)(?::\s*(?P<status>[^\]]+))?\]\s+)?(?P<mensagem>.*)$").unwrap();
        return String::from_utf8(output.stdout).unwrap().lines()
            .filter_map(|line| regex.captures(line))
            .map(BranchInfo::from)
            .collect();
    }
    vec![]
}

fn remove_deleted_branches(branches: &[BranchInfo], project_dir: Option<&Path>) {
    let gone_branches: Vec<&str> = branches.iter()
        .filter(|b| !b.current && b.gone)
        .map(|b| b.name.as_str())
        .collect();
    if gone_branches.is_empty() {
        return;
    }
    let remove_local = confirm::render(
        "Deseja remover as branches locais deletadas no remoto?",
        Some(false),
    )
    .unwrap();
    if remove_local {
        create_git_command(project_dir)
            .args(["branch", "-D"])
            .args(&gone_branches)
            .output()
            .unwrap();
    }
}

fn update_behind_branches(branches: &[BranchInfo], project_dir: Option<&Path>) {
    branches.iter()
        .filter(|b| !b.current && !b.ahead && b.behind)
        .for_each(|b| update_reference_branch(project_dir, b));
}

fn update_reference_branch(project_dir: Option<&Path>, b: &BranchInfo) {
    let branch_name = b.name.as_str();
    let remote_branch = b.remote.as_deref().unwrap();
    println!("Atualizando branch {} para {}", branch_name, remote_branch);
    create_git_command(project_dir)
        .args(vec![
            "update-ref",
            format!("refs/heads/{}", branch_name).as_str(),
            format!("refs/remotes/{}", remote_branch).as_str(),
        ])
        .output()
        .expect(format!("Não foi possível atualizar a branch {}", branch_name).as_str());
}

fn update_divergent_branches(branches: &[BranchInfo], project_dir: Option<&Path>) {
    let branches_divergentes: Vec<&BranchInfo> = branches.iter()
        .filter(|b| !b.current && b.ahead && b.behind)
        .collect();
    if !branches_divergentes.is_empty() {
        let stashed = stash_changes(project_dir);
        println!("Sincronizando branches divergentes");
        for branch in &branches_divergentes {
            merge_branch(project_dir, &branch);
        }
        checkout_current(&branches, project_dir);
        if stashed {
            create_git_command(project_dir)
                .args(["stash", "pop"])
                .output().unwrap();
        }
    }
}

fn merge_branch(project_dir: Option<&Path>, branch: &BranchInfo) {
    create_git_command(project_dir)
        .args(["checkout", branch.name.as_str()])
        .output().expect(&format!("Não foi possível fazer checkout para banch {}", branch.name.as_str()));
    let output = create_git_command(project_dir)
        .args(["merge", "--no-ff", "--no-edit", branch.remote.as_deref().unwrap()])
        .output().unwrap();
    let out_content = std::str::from_utf8(&output.stdout).unwrap();
    if out_content.contains("CONFLICT") {
        println!(r#"Não foi possível realizar o merge da branch "{}", resolva os conflitos manualmente"#, branch.name.as_str());
        create_git_command(project_dir)
            .args(["merge", "--abort"])
            .output().unwrap();
    }
}

fn checkout_current(branches_divergentes: &[BranchInfo], project_dir: Option<&Path>) {
    let current = branches_divergentes.iter()
        .find(|b| b.current)
        .unwrap();
    create_git_command(project_dir)
        .args(["checkout", current.name.as_str()])
        .output().unwrap();
}

fn stash_changes(project_dir: Option<&Path>) -> bool {
    let status_output = create_git_command(project_dir)
        .args(["status"])
        .output().expect("Erro ao verificar status do repositório");
    if status_output.status.success() {
        let content = String::from_utf8(status_output.stdout).unwrap();
        if content.contains("Changes not staged for commit:") {
            let output = create_git_command(project_dir)
                .args(["stash", "save", "-u"])
                .output()
                .expect("Não foi possível realizar o stash de forma seguras das informações não commitadas");
            return output.status.success();
        }
    }
    false
}

fn create_git_command(project_dir: Option<&Path>) -> Command {
    let mut command = Command::new("git");
    if let Some(dir) = project_dir {
        command.current_dir(dir);
    }
    command
}