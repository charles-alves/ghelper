use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use std::path::PathBuf;

use crate::domain::app_config::AppConfig;

pub fn render(app_config: &mut AppConfig) -> Result<()> {
    let theme = ColorfulTheme::default();
    let input_jira = criar_input("URL base do Jira", app_config.jira.as_deref(), &theme);
    app_config.jira = Some(input_jira.interact_text()?);
    let input_git = criar_input("URL base do Git", app_config.git.as_deref(), &theme);
    app_config.git = Some(input_git.interact_text()?);
    let input_workspace = criar_input("Diretório base para armazenamento dos projetos", app_config.workspace.to_str(), &theme);
    app_config.workspace = PathBuf::from(input_workspace.interact_text()?);
    Ok(())
}

fn criar_input<'a>(prompt: &'a str, default: Option<&'a str>, theme: &'a ColorfulTheme) -> Input<'a, String> {
    let input = Input::<String>::with_theme(theme)
        .with_prompt(prompt);
    if let Some(default) = default {
        return input.default(default.to_string());
    }
    input
}