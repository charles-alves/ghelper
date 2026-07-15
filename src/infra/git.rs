use crate::os;
use crate::os::exec_output::ExecOutput;
use crate::os::exec_output::ExecOutput::Success;
use crate::view::select;

use regex::Regex;

pub enum BranchLocation {
    Local, Remote(String), None
}

pub fn checkout(branch_name: &str) {
    let checkout_command = match exists_branch(branch_name) {
        BranchLocation::Local => {
            println!("Checkout realizado na branch {}", branch_name);
            format!("git checkout {}", branch_name)
        }
        BranchLocation::Remote(remote) => {
            let remote_branch = format!("{}/{}", remote, branch_name);
            println!("Truncando branch local com {}", &remote_branch);
            format!("git checkout -t -b {} {}", branch_name, &remote_branch)
        },
        BranchLocation::None => {
            println!("Criando nova branch local {}", branch_name);
            format!("git checkout -b {}", branch_name)
        },
    };
    os::execute(&checkout_command, None)
        .expect(&format!("Não foi possível finalizar o checkout para branch {}", branch_name));
}

pub fn pwd() -> String {
    let branch_output = os::execute("git branch --show-current", None)
        .expect("Não foi possível recuperar o nome da branch atual");
    match branch_output {
        Success(output) => output.trim().to_string(),
        ExecOutput::Failure(outerr) => panic!("{outerr}")
    }
}

pub fn local_branches() -> Vec<String> {
    let result = os::execute("git branch --format \"%(refname:short)\"", None)
        .expect("Não foi possível identificar branches no diretório atual");
    if let Success(output) = result {
        output.lines()
            .map(String::from)
            .collect()
    } else {
        vec![]
    }
}

pub fn exists_branch(branch_name: &str) -> BranchLocation {
    if exists_local(branch_name) {
        return BranchLocation::Local
    } else if let Some(remote) = exists_remote(branch_name) {
        return BranchLocation::Remote(remote)
    }
    BranchLocation::None
}

fn exists_local(branch_name: &str) -> bool {
    let output = execute_local_branch_list();
    let branch_regx = Regex::new(format!(r"^[\s*]\s{}$", regex::escape(branch_name)).as_str())
        .expect("Falha ao criar regex para branches locais");
    if let Success(output) = output {
        output.lines().any(|line| branch_regx.is_match(line))
    } else {
        panic!("Falha ao tentar recuperar branches locais");
    }
}

fn execute_local_branch_list() -> ExecOutput {
    os::execute("git branch", None)
        .expect("Falha ao tentar recuperar branches locais")
}

fn exists_remote(branch_name: &str) -> Option<String> {
    let output = execute_remote_branch_list();
    if let Success(output) = output {
        let remote_branches = get_match_remote(output, branch_name);
        if remote_branches.len() == 1 {
            return Some(remote_branches[0].to_string())
        }
        if !remote_branches.is_empty() {
            return Some(select_remote(&remote_branches));
        }
    } else {
        panic!("Falha ao tentar recuperar branches locais");
    }
    None
}

fn execute_remote_branch_list() -> ExecOutput {
    os::execute("git branch -r", None)
        .expect("Falha ao tentar recuperar branches locais")
}

fn get_match_remote(output: String, branch_name: &str) -> Vec<String> {
    let branch_regx = Regex::new(format!(r"^\s\s(.+?)/{}$", branch_name).as_str())
        .expect("Falha ao criar regex para branches locais");
    output.lines()
        .filter_map(|b| branch_regx.captures(b).map(|m| m[1].to_string()))
        .collect()
}

fn select_remote(remote_branches: &[String]) -> String {
    select::render(
        "A branch solicitada existe em mais de um remoto, selecione o desejado",
        &remote_branches,
        Some("origin")
    )
        .expect("Não foi possível executar a seleção de remotos")
}