use anyhow::{bail, Result};
use std::process::Command;

use crate::cli::checkout_branch_args::BranchType;
use crate::infra::git;

pub(crate) fn run(branch_type: Option<BranchType>, branch: Option<&str>, native_args: &Vec<String>) -> Result<()> {
    if branch == Some("-") {
        if branch_type.is_some() {
            bail!("Não é permitido utilizar o parâmetro \"-\" junto com um modificador de branch")
        }
        Command::new("git").arg("checkout").arg("-").status()?;
        return Ok(());
    }
    if !native_args.is_empty() {
        Command::new("git").arg("checkout").args(native_args).status()?;
        return Ok(());
    }
    if branch.is_none() {
        bail!("É necessário informar o nome da branch que deseja realizar checkout")
    }
    let branch_name = branch_type.map(|t| {format!("{}/{}", t, branch.unwrap())})
        .unwrap_or_else(|| branch.unwrap().to_string());
    git::checkout(&branch_name);
    Ok(())
}