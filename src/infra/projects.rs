use glob::glob;

use crate::infra::project_files;

pub fn lisit() -> anyhow::Result<Vec<String>> {
    let workspace = project_files::load_config()?.workspace;
    let pattern = format!("{}/*/*/", workspace.to_string_lossy());
    let result = glob(&pattern)?.filter_map(anyhow::Result::ok)
        .map(|e| e.strip_prefix(&workspace).unwrap().to_owned())
        .map(|e| e.to_string_lossy().to_string())
        .collect();
    Ok(result)
}