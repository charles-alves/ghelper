use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub fn render(prompt: &str, itens: &Vec<String>, default: Option<String>, theme: &ColorfulTheme) -> Result<usize> {
    let index = default.and_then(|d| itens.iter().position(|i| i == &d));
    Select::with_theme(theme)
        .with_prompt(prompt)
        .default(index.unwrap_or(0))
        .items(itens)
        .max_length(2)
        .interact()
        .context("Não foi possível confimar o resultado")
}