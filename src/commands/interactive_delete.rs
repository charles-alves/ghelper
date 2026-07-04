use anyhow::{bail, Result};
use dialoguer::theme::ColorfulTheme;
use std::process::Command;

use crate::infra::git;
use crate::view::{confirm, mult_select};

pub fn run() -> Result<()> {
    let current_branch = git::pwd();
    let branches = git::local_branches().iter()
        .filter(|b| *b != &current_branch)
        .map(String::from)
        .collect::<Vec<String>>();
    if branches.is_empty() {
        bail!("Não existem branches disponíveis para serem deletadas")
    }
    let selected = mult_select::render(
        "Quais branches serão deletadas?",
            &branches,
            &vec![],
            &ColorfulTheme::default()
    )?;
    let confirmed = confirm::render(
        "Tem certeza que deseja apagar as branches selecionadas",
        Some(false),
        &ColorfulTheme::default()
    )?;
    if confirmed {
        for branch in selected {
            Command::new("git")
                .args(vec!["branch", "-D", &branch])
                .status()?;
        }
    }
    Ok(())
}