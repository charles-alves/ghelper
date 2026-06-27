use anyhow::Result;
use glob::glob;

use crate::infra::project_files;

pub fn list_filter(filter: &Option<String>) -> Result<Vec<String>> {
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
        .map(|e| e.to_string_lossy().to_string())
        .collect();
    Ok(result)
}