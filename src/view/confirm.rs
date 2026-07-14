use anyhow::{Context, Result};
use dialoguer::Confirm;

pub fn render(prompt: &str, default: Option<bool>) -> Result<bool> {
    let mut confirm = Confirm::new()
        .with_prompt(prompt);
    if let Some(default) = default {
        confirm = confirm.default(default);
    }
    confirm.interact().context("Não foi possível confimar o resultado")
}