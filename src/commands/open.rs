use crate::commands::sync;
use crate::infra::projects;
use crate::os;
use crate::view::mult_select;

use anyhow::Result;

pub(crate) fn run(update: &bool, filter: &Option<String>) -> Result<()> {
    let selected = select_projects(filter);
    if *update {
        sync_projects(&selected);
    }
    for project in &selected {
        os::open_ide(&project)?;
    }
    Ok(())
}

fn select_projects(filter: &Option<String>) -> Vec<String> {
    let projects = projects::list_filter(filter)
        .expect("Não foi possível listar projetos do Workspace");
    if projects.is_empty() {
        println!("Não foram encontrados projetos para serem abertos");
        return vec![];
    }
    mult_select::render(
        "Selecione os projetos que deseja abrir",
        &projects,
        &vec![],
    )
        .expect("Não foi possível finalizar a seleção de projetos")
}

fn sync_projects(selected: &[String]) {
    for project in selected {
        println!("Atualizando o projeto {}", project);
        if let Err(_) = sync::run(Some(&project)) {
            println!("Não foi possível executar a atualização do projeto {}", project);
        }
    }
}
