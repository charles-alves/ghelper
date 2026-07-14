use anyhow::Result;
use glob::glob;
use std::path::PathBuf;

use crate::infra::project_files;

pub fn list_filter(filter: Option<&str>) -> Result<Vec<String>> {
    let result = match filter {
        None => list()?,
        Some(filter) => list()?.iter()
            .filter(|p| p.contains(filter))
            .cloned()
            .collect()
    };
    Ok(result)
}

pub fn list() -> Result<Vec<String>> {
    let workspace = project_files::load_config()?.workspace;
    let pattern = format!("{}/*/*/", workspace.to_string_lossy());
    let result = glob(&pattern)?.filter_map(anyhow::Result::ok)
        .map(|e| e.strip_prefix(&workspace).unwrap().to_owned())
        .map(|e| e.to_string_lossy().replace("\\", "/"))
        .collect();
    Ok(result)
}

pub fn dir(project: &str) -> Option<PathBuf> {
    if let Ok(config) = project_files::load_config() {
        let project_dir = config.workspace.join(project);
        if project_dir.exists() {
            return Some(project_dir);
        }
    }
    None
}