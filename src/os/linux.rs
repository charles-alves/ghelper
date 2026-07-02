use anyhow::{Context, Result};
use std::process::Command;

use crate::infra::project_files;
use crate::os::exec_output::ExecOutput;

pub fn execute(command: &str, repo: Option<&str>) -> Result<ExecOutput> {
    init_command(repo)?
        .arg(command)
        .output()
        .map(ExecOutput::from)
        .context("Não foi possível executar o comando solicitado")
}

pub fn execute_forgot(command: &str, repo: Option<&str>) -> Result<()> {
    init_command(repo)?
        .arg(format!("{} > /dev/null 2>&1 &", command))
        .spawn()
        .context("Não foi possível executar o comando solicitado")
        .map(|_| ())
}

fn init_command(repo: Option<&str>) -> Result<Command> {
    let mut command = Command::new("sh");
    command.arg("-c");
    if let Some(repo) = repo {
        command.current_dir(project_files::load_config()?.workspace.join(repo));
    }
    Ok(command)
}
