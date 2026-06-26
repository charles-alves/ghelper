use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub fn render(prompt: &str, itens: &Vec<String>, default: Option<usize>, theme: &ColorfulTheme) -> Result<usize> {
    let mut confirm = Select::with_theme(theme)
        .with_prompt(prompt);
    if let Some(default) = default {
        confirm = confirm.default(default);
    }
    confirm.items(itens)
        .max_length(2)
        .interact()
        .context("Não foi possível confimar o resultado")
}