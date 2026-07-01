use anyhow::Result;
use std::process::Command;

pub fn execute(command: &str) -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()?;
    Ok(())
}