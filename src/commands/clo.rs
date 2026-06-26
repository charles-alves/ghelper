use anyhow::{bail, Result};
use dialoguer::theme::ColorfulTheme;
use regex::Regex;
use std::path::PathBuf;
use std::{fs, process::Command};
use url::Url;

use crate::infra::project_files;
use crate::view::confirm;

pub(crate) fn run(repo: &str) -> Result<()> {
    let project = match get_repo_values(repo)? {
        Some(project) => project,
        None => anyhow::bail!("Não foi possível identificar o grupo e nome do projeto"),
    };
    let group_dir = project_files::workspace()?.join(&project.0);
    if group_dir.join(&project.1).exists() {
        println!("Já existe o projeto localmente");
        let execut_sync = confirm::render("Deseja atualizar o repositório?", Some(true), &ColorfulTheme::default())?;
        if execut_sync {
            println!("Executar Sincroniazação");
        }
    } else {
        println!("Clonando projeto");
        fs::create_dir_all(&group_dir)?;
        return if project.2 {
            clone(repo, &group_dir)
        } else {
            let url = get_repo_url(&project)?;
            clone(&url.as_str(), &group_dir)
        }
    }
    Ok(())
}

fn get_repo_values(repo: &str) -> Result<Option<(String, String, bool)>> {
    let regex = Regex::new(r"\.git$")?;
    if regex.is_match(repo) {
        let regex = Regex::new(r".+[/:](\S+)/(\S+)\.git$")?;
        if let Some(caps) = regex.captures(repo) {
            return Ok(Some((caps[1].to_string(), caps[2].to_string(), true)));
        }
    } else {
        let split: Vec<&str> = repo.split("/").collect();
        return Ok(Some((split[0].to_string(), split[1].to_string(), false)));
    }
    Ok(None)
}

fn clone(url: &str, group_dir: &PathBuf) -> Result<()> {
    Command::new("git")
        .current_dir(group_dir)
        .arg("clone")
        .arg(url)
        .status()?;
    Ok(())
}

fn get_repo_url(project: &(String, String, bool)) -> Result<Url> {
    match project_files::load_config()?.git {
        Some(git_url) => {
            let mut url = Url::parse(&git_url)?;
            {
                let mut segments = url.path_segments_mut().unwrap();
                segments.push(&project.0);
                segments.push(&format!("{}.git", project.1));
            }
            Ok(url)
        }
        None => {
            bail!("Não foi possível localizar a URL base para o Git nas configurações do aplicativo")
        }
    }
}
