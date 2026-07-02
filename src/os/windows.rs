use anyhow::{Context, Result};
use std::process::Command;

use crate::infra::project_files;
use crate::os::exec_output::ExecOutput;

pub fn execute(command: &str, repo: Option<&str>) -> Result<ExecOutput> {
    Command::new(command).output()
        .map(ExecOutput::from)
        .context("Não foi possível executar o comando solicitado")
}

pub fn execute_forgot(command: &str, repo: Option<&str>) -> Result<()> {
    init_command(repo)?
        .arg(format!("{{ {} }} > null", command))
        .spawn()
        .context("Não foi possível executar o comando solicitado")
        .map(|_| ())
}

fn init_command(repo: Option<&str>) -> Result<Command> {
    let mut command = Command::new("Start-Job");
    command.arg("-ScriptBlock");
    if let Some(repo) = repo {
        command.current_dir(project_files::load_config()?.workspace.join(repo));
    }
    Ok(command)
}
