use anyhow::Result;
use std::path::PathBuf;
use crate::infra::project_files;
use crate::view::config_form_view;

pub fn run(jira: &Option<Option<String>>, git: &Option<Option<String>>, workspace: &Option<Option<String>>) -> Result<()> {
    let mut config = project_files::load_config()?;
    if let Some(jira_config) = jira {
        match jira_config {
            Some(jira_config) => {config.jira = Some(jira_config.to_string())}
            None => {
                println!("{}", config.jira.as_deref().unwrap_or("URL base do Jira não informada"));
            }
        };
    }
    if let Some(git_config) = git {
        match git_config {
            Some(git_config) => {config.git = Some(git_config.to_string())}
            None => {
                println!("{}", config.git.as_deref().unwrap_or("URL base do Git não informada"));
            }
        };
    }
    if let Some(workspace_config) = workspace {
        match workspace_config {
            Some(workspace_config) => {config.workspace = PathBuf::from(workspace_config)}
            None => {println!("{:#?}", config.workspace)}
        }
    }
    if jira.is_none() && git.is_none() && workspace.is_none() {
        config_form_view::render(&mut config)?;
    }
    project_files::save_config(&config)
}