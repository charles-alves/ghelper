use crate::infra::projects;
use crate::view::select;

use anyhow::Result;
use arboard::Clipboard;

pub(crate) fn run(filter: &Option<String>) -> Result<()> {
    let projects = projects::list_filter(filter)?; 
    let selected = select::render("Selecione o projeto desejado", &projects, None)?;
    Clipboard::new()?.set_text(selected)?;
    Ok(())
}
