mod cli;
mod commands;
mod infra;
mod domain;
pub mod view;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use crate::cli::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Config { jira, git, workspace } => commands::config::run(jira, git, workspace)?,
        Command::Branchs { .. } => {}
        Command::Tags { .. } => {}
        Command::Clone { .. } => {}
        Command::InteractiveCheckout { .. } => {}
        Command::InteractiveDelete { .. } => {}
        Command::Up { .. } => {}
        Command::Projects { .. } => {}
        Command::Search { .. } => {}
        Command::Console { .. } => {}
        Command::StashList { .. } => {}
        Command::StashSave { .. } => {}
        Command::StashApply { .. } => {}
        Command::StashPop { .. } => {}
    }
    Ok(())
}
