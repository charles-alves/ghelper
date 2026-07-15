use anyhow::{bail, Result};
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
        &vec![]
    )?;
    let confirmed = confirm::render(
        "Tem certeza que deseja apagar as branches selecionadas",
        Some(false)
    )?;
    if confirmed {
        Command::new("git")
            .args(["branch", "-D"])
            .args(&selected)
            .status()?;
    }
    Ok(())
}