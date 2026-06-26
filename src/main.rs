mod cli;
mod commands;
mod infra;
mod domain;
pub mod view;

use crate::cli::Command;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Config { jira, git, workspace } => commands::config::run(jira, git, workspace)?,
        Command::Clone { repo } => commands::clo::run(repo)?,
        Command::Branchs { .. } => {}
        Command::Tags { .. } => {}
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
