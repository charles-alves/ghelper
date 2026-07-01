use crate::infra::project_files;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux as target;


#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
use windows as target;

pub use target::execute;

pub fn open_ide(project_name: &str) -> anyhow::Result<()> {
    let config = project_files::load_config()?;
    let command = format!("{} \"{:#?}\"", config.ide_executable.as_ref().unwrap(), config.workspace.join(project_name));
    execute(&command)
}