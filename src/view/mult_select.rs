use anyhow::Result;
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;

pub fn render(
    prompt: &str,
    itens: &[String],
    default: &[String],
    theme: &ColorfulTheme,
) -> Result<Vec<String>> {
    let mut confirm = MultiSelect::with_theme(theme).with_prompt(prompt);
    if !default.is_empty() {
        let defaults: Vec<bool> = itens.iter().map(|v| default.contains(v)).collect();
        confirm = confirm.defaults(&defaults);
    }
    let selected = confirm.items(itens).max_length(10).interact()?;
    let result = selected.iter().map(|&v| itens[v].clone()).collect();
    Ok(result)
}
