use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub fn render<'a>(prompt: &'a str, default: Option<&'a str>, theme: &'a ColorfulTheme) -> Input<'a, String> {
    let input = Input::<String>::with_theme(theme)
        .with_prompt(prompt);
    if let Some(default) = default {
        return input.default(default.to_string());
    }
    input
}