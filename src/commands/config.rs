use crate::cli::ConfigArgs;
use crate::infra::project_files;
use crate::view::config_form_view;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(args: &ConfigArgs) -> Result<()> {
    let mut config = project_files::load_config()?;
    if let Some(jira_config) = &args.jira {
        match jira_config {
            Some(jira_config) => config.jira = Some(jira_config.to_string()),
            None => {
                println!("{}", config.jira.as_deref().unwrap_or("URL base do Jira não informada"));
            }
        };
    }
    if let Some(git_config) = &args.git {
        match git_config {
            Some(git_config) => config.git = Some(git_config.to_string()),
            None => {
                println!("{}", config.git.as_deref().unwrap_or("URL base do Git não informada"));
            }
        };
    }
    if let Some(workspace_config) = &args.workspace {
        match workspace_config {
            Some(workspace_config) => config.workspace = PathBuf::from(workspace_config),
            None => {
                println!("{:#?}", config.workspace)
            }
        }
    }
    if let Some(ide_config) = &args.ide {
        match ide_config {
            Some(ide_config) => config.ide_executable = Some(ide_config.to_string()),
            None => {
                println!("{}", config.ide_executable.as_deref().unwrap_or("Comando para abertura da IDE não configurada"));
            }
        }
    }
    if args.is_empty() {
        config_form_view::render(&mut config)?;
    }
    project_files::save_config(&config)
}
