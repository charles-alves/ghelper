use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use std::path::PathBuf;

use crate::domain::app_config::AppConfig;
use crate::view::input;

pub fn render(app_config: &mut AppConfig) -> Result<()> {
    let theme = ColorfulTheme::default();
    let input_jira = input::render("URL base do Jira", app_config.jira.as_deref(), &theme);
    app_config.jira = Some(input_jira.interact_text()?);
    let input_git = input::render("URL base do Git", app_config.git.as_deref(), &theme);
    app_config.git = Some(input_git.interact_text()?);
    let input_workspace = input::render("Diretório base para armazenamento dos projetos", app_config.workspace.to_str(), &theme);
    app_config.workspace = PathBuf::from(input_workspace.interact_text()?);
    Ok(())
}
