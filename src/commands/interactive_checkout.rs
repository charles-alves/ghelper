use crate::os;
use crate::os::exec_output::ExecOutput::{Failure, Success};
use crate::view::select;

use anyhow::{bail, Result};
use regex::Regex;
use std::collections::HashSet;
use dialoguer::theme::ColorfulTheme;

pub(crate) fn run() -> Result<()> {
    let remote_prefix_rx = Regex::new(r"remotes/\S+?/")?;
    let current_branch;
    let output: Vec<String> = match os::execute("git branch -a", None)? {
        Success(output) => {
            let mut output_lines = output.lines();
            current_branch = output_lines.find(|b| b.starts_with("*"))
                .map(|b| b.trim_start_matches("* ").to_string());
            output_lines.filter_map(|b| {
                    if b.starts_with("*") || b.contains("HEAD") {
                        return None;
                    }
                    return Some(remote_prefix_rx.replace(b.trim(), "").to_string());
                })
                .collect::<HashSet<String>>()
                .into_iter()
                .collect()
        }
        Failure(outerr) => bail!(outerr)
    };
    let selected_branch = select::render("Selecione a branch que deseja realizar checkout", &output, current_branch.as_deref(), &ColorfulTheme::default())?;
    if current_branch.as_deref() != Some(&selected_branch) {
        os::execute(&format!("git checkout {}", &selected_branch), None)?;
    }
    Ok(())
}