use anyhow::{Context, Result};
use std::process::Command;

pub fn execute(command: &str) -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("{} > /dev/null 2>&1 &", command))
        .spawn()
        .context("Não foi possível executar o comando solicitado")
        .map(|_| ())
}