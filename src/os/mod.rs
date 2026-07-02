#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux as target;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
use windows as target;

pub mod exec_output;

use crate::infra::project_files;

pub use target::*;

pub fn open_ide(project_name: &str) -> anyhow::Result<()> {
    let config = project_files::load_config()?;
    let command = format!("{} .", config.ide_executable.as_ref().unwrap());
    execute_forgot(&command, Some(project_name))
}