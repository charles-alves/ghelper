use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "gh",
    about = "Auxilia na utilização do Git, com alias e tarefas automatizadas"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Args)]
pub struct ConsoleArgs {
    #[arg(short, long, conflicts_with_all = ["database", "kubernates", "kafka"])]
    pub consul: bool,
    #[arg(short = 'b', long, conflicts_with_all = ["consul", "kubernates", "kafka"])]
    pub database: bool,
    #[arg(short, long, conflicts_with_all = ["database", "consul", "kafka"])]
    pub kubernates: bool,
    #[arg(short = 't', long, conflicts_with_all = ["database", "kubernates", "consul"])]
    pub kafka: bool,
    #[arg(short, long, conflicts_with_all = ["homolog", "stress", "producao"])]
    pub develop: bool,
    #[arg(short = 'o', long, conflicts_with_all = ["develop", "stress", "producao"])]
    pub homolog: bool,
    #[arg(short, long, conflicts_with_all = ["homolog", "develop", "producao"])]
    pub stress: bool,
    #[arg(short, long, conflicts_with_all = ["homolog", "stress", "develop"])]
    pub producao: bool,
    pub repo: Option<String>
}

#[derive(Subcommand)]
pub enum Command {
    Config {
        #[arg(short, long)]
        jira: Option<Option<String>>,
        #[arg(short, long)]
        git: Option<Option<String>>,
        #[arg(short, long)]
        workspace: Option<Option<String>>,
    },
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
    /// Lista as branches existentes para o repositório selecionado
    #[command(name = "b")]
    Branchs {
        repo: Option<String>
    },
    /// Lista as branches existentes para o repositório em ordem decrescente
    #[command(name = "t")]
    Tags {
        repo: Option<String>
    },
    /// Realiza o checkout interativo em uma branch do repositório
    /// caso a branch exista somente no remoto ela será criada localmente
    #[command(name = "ci")]
    InteractiveCheckout {},
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