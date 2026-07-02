use crate::os;
use crate::os::exec_output::ExecOutput::Success;
use crate::infra::git;

use std::process::Command;
use anyhow::{bail, Result};

pub(crate) fn run() -> Result<()> {
    if let Success(output) = os::execute("git status -sb", None)? {
        if output.contains("...") {
            Command::new("git").arg("push").status()?;
        } else {
            if let Success(output) = os::execute("git remote", None)? {
                let remotes = output.lines().map(String::from).collect::<Vec<String>>();
                if remotes.len() == 1 {
                    Command::new("git").arg("push")
                        .arg("-u")
                        .arg(&remotes[0])
                        .arg(git::pwd())
                        .status()?;
                }
            }
        }
    } else {
        bail!("Não foi possível recuperar as informações da branch atual")
    }
    Ok(())
}