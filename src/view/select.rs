use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub fn render(prompt: &str, itens: &Vec<String>, default: Option<String>, theme: &ColorfulTheme) -> Result<String> {
    let index = default.and_then(|d| itens.iter().position(|i| i == &d));
    let selected_index = Select::with_theme(theme)
        .with_prompt(prompt)
        .default(index.unwrap_or(0))
        .items(itens)
        .max_length(2)
        .interact()
        .context("Não foi possível confimar o resultado")?;
    Ok(itens[selected_index].to_string())
}