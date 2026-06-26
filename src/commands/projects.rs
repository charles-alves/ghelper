use crate::infra::projects;
use crate::view::select;

use anyhow::Result;
use arboard::Clipboard;
use dialoguer::theme::ColorfulTheme;

pub(crate) fn run(filter: &Option<String>) -> Result<()> {
    let projects:Vec<String> = match filter {
        None => projects::lisit()?,
        Some(filter) => projects::lisit()?.iter()
                .filter(|p| p.contains(filter))
                .cloned()
                .collect()

    };
    let selected = select::render("Selecione o projeto desejado", &projects, None, &ColorfulTheme::default())?;
    Clipboard::new()?.set_text(selected)?;
    Ok(())
}
