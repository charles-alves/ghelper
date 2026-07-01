use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::infra::project_files;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub jira: Option<String>,
    pub git: Option<String>,
    pub workspace: PathBuf,
    pub ide_executable: Option<String>
}

impl Default for AppConfig {
    fn default() -> Self {
        let workspace = project_files::workspace()
            .expect("Não foi possível identificar o Workspace");
        Self {
            jira: None,
            git: None,
            workspace,
            ide_executable: None
        }
    }
}

