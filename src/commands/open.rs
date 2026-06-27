use crate::infra::{projects, project_files};
use crate::view::mult_select;
use crate::commands::sync;

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use std::process::Command;

pub(crate) fn run(update: &bool, filter: &Option<String>) -> Result<()> {
    let selected = select_projects(filter);
    if *update {
        sync_projects(&selected);
    }
    Command::new("bash").current_dir(project_files::workspace()?.join(&selected[0])).status()?;
    Ok(())
}

fn select_projects(filter: &Option<String>) -> Vec<String> {
    let projects = projects::list_filter(filter)
        .expect("Não foi possível listar projetos do Workspace");
    mult_select::render(
        "Selecione os projetos que deseja abrir",
        &projects,
        vec![],
        &ColorfulTheme::default(),
    )
        .expect("Não foi possível finalizar a seleção de projetos")
}

fn sync_projects(selected: &Vec<String>) {
    for project in selected {
        println!("Atualizando o projeto {}", project);
        if let Err(_) = sync::run(Some(&project)) {
            println!("Não foi possível executar a atualização do projeto {}", project);
        }
    }
}
