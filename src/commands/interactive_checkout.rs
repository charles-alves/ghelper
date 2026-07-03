use crate::infra::git;
use crate::os;
use crate::os::exec_output::ExecOutput::{Failure, Success};
use crate::view::select;

use anyhow::{bail, Result};
use dialoguer::theme::ColorfulTheme;
use regex::Regex;
use std::collections::HashSet;

pub(crate) fn run() -> Result<()> {
    let current_branch;
    let branches: Vec<String> = match os::execute("git branch -a", None)? {
        Success(output) => {
            current_branch = map_current_branch(&output);
            map_branches(&output)
        }
        Failure(outerr) => bail!(outerr)
    };
    let selected_branch = select::render(
        "Selecione a branch que deseja realizar checkout",
        &branches,
        current_branch.as_deref(),
        &ColorfulTheme::default()
    )?;
    if current_branch.as_deref() != Some(&selected_branch) {
        git::checkout(&selected_branch);
    }
    Ok(())
}

fn map_current_branch(output: &String) -> Option<String> {
    output.lines().find(|b| b.starts_with("*"))
        .map(|b| b.trim_start_matches("* ").to_string())
}

fn map_branches(output: &String) -> Vec<String> {
    let remote_prefix_rx = Regex::new(r"remotes/\S+?/").unwrap();
    output.lines().filter_map(|b| {
        if b.starts_with("*") || b.contains("HEAD") {
            return None;
        }
        return Some(remote_prefix_rx.replace(b.trim(), "").to_string());
    })
        .collect::<HashSet<String>>()
        .into_iter()
        .collect()
}