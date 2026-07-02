pub mod console_args;
pub mod config_args;
pub mod checkout_branch_args;

use clap::{Parser, Subcommand};

use crate::cli::checkout_branch_args::CheckoutBranchArgs;
use crate::cli::config_args::ConfigArgs;
use crate::cli::console_args::ConsoleArgs;

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
    /// Realiza o delete interativo de branchs locais do repositório
    #[command(name = "di")]
    InteractiveDelete {},
    /// Executa o push para o remoto, independente da branch já existir
    Up {},
    /// Abre o repositório no browser na tela de busca
    Search {
        /// Filtro utilizado para realizar a busca
        filter: Option<String>
    },
    /// Abre o devconsole para um projeto e páginas especificadas via parâmetros
    Console(ConsoleArgs),
    /// Lista os commits em stash para o repositório
    #[command(name = "sl")]
    StashList {},
    /// Executa o stash para os arquivos alterados no repositório
    #[command(name = "ss")]
    StashSave {
        /// Mantem os arquivos que encontram em stage
        k: Option<bool>,
        /// Mensagem para facilitar a identificação do Stash
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Executa o stash apply para os arquivos alterados no repositório
    #[command(name = "sa")]
    StashApply {
        /// Índice do stash desejado
        #[arg(short, long)]
        index: Option<i8>,
    },
    /// Executa o stash pop para os arquivos alterados no repositório
    #[command(name = "sp")]
    StashPop {
        /// Índice do stash desejado
        #[arg(short, long)]
        index: Option<i8>,
    }
}