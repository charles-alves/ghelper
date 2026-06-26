use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

pub fn render(prompt: &str, default: Option<bool>, theme: &ColorfulTheme) -> Result<bool> {
    let mut confirm = Confirm::with_theme(theme)
        .with_prompt(prompt);
    if let Some(default) = default {
        confirm = confirm.default(default);
    }
    confirm.interact().context("Não foi possível confimar o resultado")
}