mod cli;
mod commands;
mod infra;
mod domain;
pub mod view;
pub mod os;

use crate::cli::Command;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // println!("{:#?}", &cli.command);
    match &cli.command {
        Command::Config(args) => commands::config::run(args)?,
        Command::Clone { repo } => commands::clo::run(repo)?,
        Command::Projects { filter } => commands::projects::run(filter)?,
        Command::Open { update, filter } => commands::open::run(update, filter)?,
        Command::Checkout { branch_type, branch, native_args } =>
            commands::checkout::run(branch_type.value(), branch.as_deref(), native_args)?,
        Command::InteractiveCheckout => commands::interactive_checkout::run()?,
        Command::Up { force } => commands::up::run(force.clone())?,
        Command::InteractiveDelete => commands::interactive_delete::run()?
    }
    Ok(())
}
