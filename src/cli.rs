pub mod console_args;
pub mod config_args;
pub mod checkout_branch_args;

use clap::{Parser, Subcommand};

use crate::cli::checkout_branch_args::CheckoutBranchArgs;
use crate::cli::config_args::ConfigArgs;

#[derive(Parser)]
#[command(
    name = "gh",
    about = "Auxilia na utilização do Git, com alias e tarefas automatizadas"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Config(ConfigArgs),
    /// Clona um novo repositório ao workspace
    #[command(name = "clo")]
    Clone {
        /// Nome do repositório que será clonado
        repo: String
    },
    /// Lista os projetos clonados
    #[command(name = "pjts")]
    Projects {
        /// Possibilita filtrar os projetos
        filter: Option<String>
    },
    /// Lista os projetos existentes localmente e abre utilizando a IDE configurada no sistema
    Open {
        #[arg(short, long)]
        update: bool,
        filter: Option<String>
    },
    /// Realiza o checkout em uma branch do repositório,
    /// caso a branch não exista será criada e caso esteja no
    /// remoto, será criada com o devido bind
    #[command(name = "c")]
    Checkout {
        /// O nome da branch ou '-' para navegar para branch anterior
        branch: Option<String>,
        #[command(flatten)]
        branch_type: CheckoutBranchArgs,
        /// Argumentos nativos do git passados após '--'
        #[arg(last = true, conflicts_with = "branch")]
        native_args: Vec<String>,
    },
    /// Realiza o checkout interativo em uma branch do repositório
    /// caso a branch exista somente no remoto ela será criada localmente
    #[command(name = "ci")]
    InteractiveCheckout,
    /// Executa o push para o remoto, independente da branch já existir
    Up {
        #[arg(short, long)]
        force: bool,
    },
    /// Realiza o delete interativo de branchs locais do repositório
    #[command(name = "di")]
    InteractiveDelete,
}