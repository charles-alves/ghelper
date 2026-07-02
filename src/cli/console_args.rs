use clap::Args;

#[derive(Args, Debug)]
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