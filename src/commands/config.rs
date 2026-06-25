use std::path::PathBuf;
use anyhow::Result;

use crate::infra::project_files;
use crate::view::config_form_view;

pub fn run(jira: &Option<String>, git: &Option<String>, workspace: &Option<String>) -> Result<()> {
    let mut config = project_files::load_config()?;
    if let Some(jira_config) = jira {
        config.jira = Some(jira_config.to_string());
    }
    if let Some(git_config) = git {
        config.git = Some(git_config.to_string());
    }
    if let Some(workspace_config) = workspace {
        config.workspace = PathBuf::from(workspace_config);
    }
    if [&jira, &git, &workspace].iter().all(|x| x.is_none()) {
        config_form_view::render(&mut config)?;
    }
    Ok(())
}