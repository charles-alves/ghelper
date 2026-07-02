use crate::infra::git;
use crate::os;
use crate::os::exec_output::ExecOutput::Success;
use crate::view::select;

use anyhow::{bail, Context, Result};
use dialoguer::theme::ColorfulTheme;
use std::process::Command;

pub(crate) fn run(force: bool) -> Result<()> {
    if let Success(output) = os::execute("git status -sb", None)? {
        if output.contains("...") {
            let mut command = Command::new("git");
            command.arg("push");
            if force {
                command.arg("-f");
            }
            command.status()?;
        } else {
            if let Success(output) = os::execute("git remote", None)? {
                let remotes = output.lines().map(String::from).collect::<Vec<String>>();
                if remotes.len() == 1 {
                    truncate_to_remote(&remotes[0], force)?;
                } else if !remotes.is_empty() {
                    let remote = select::render(
                        "Selecione um remoto para subir a branch",
                        &remotes,
                        Some("origin"),
                        &ColorfulTheme::default(),
                    )?;
                    truncate_to_remote(&remote, force)?;
                } else {
                    bail!("Não foi possível identificar um remoto para subir as alterações")
                }
            }
        }
    } else {
        bail!("Não foi possível recuperar as informações da branch atual")
    }
    Ok(())
}

fn truncate_to_remote(remote: &str, force: bool) -> Result<()> {
    let mut command = Command::new("git");
    command.arg("push")
        .arg("-u")
        .arg(remote)
        .arg(git::pwd());
    if force {
        command.arg("-f");
    }
    command.status()
        .map(|_| ())
        .context("Não foi possível executar o push para o remoto")
}
