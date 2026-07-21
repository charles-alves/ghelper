use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::domain::app_config::AppConfig;

pub fn load_config() -> Result<AppConfig> {
    let config_file = project_dir()?.join("config.toml");
    if config_file.exists() {
        if let Ok(contents) = fs::read_to_string(config_file) {
            return toml::from_str::<AppConfig>(&contents).context("Falha ao realizar leitura do arquivo de configuração");
        }
    }
    Ok(AppConfig::default())
}

pub fn workspace() -> Result<PathBuf> {
    Ok(project_dir()?.join("workspace"))
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_file = project_dir()?.join("config.toml");
    let content = toml::to_string_pretty(config)?;
    fs::write(config_file, content)?;
    Ok(())
}

fn project_dir() -> Result<PathBuf> {
    if let Some(proj_dir) = ProjectDirs::from("br", "acidco","ghelper") {
        let path = proj_dir.config_dir();
        fs::create_dir_all(path).expect("Não foi possível criar o diretório de configuração");
        return Ok(path.to_path_buf());
    }
    bail!("Não foi possível localizar o diretório de configuração da aplicação")
}