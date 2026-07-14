use anyhow::Result;
use dialoguer::Input;

pub fn render(prompt: &str, default: Option<&str>) -> Result<Option<String>> {
    let mut input = Input::<String>::new()
        .with_prompt(prompt);
    if let Some(default) = default {
        input = input.default(default.to_string());
    }
    let value = input.allow_empty(true).interact_text()?;
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}