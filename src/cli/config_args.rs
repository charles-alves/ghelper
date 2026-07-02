use clap::Args;

#[derive(Args)]
pub struct ConfigArgs {
    #[arg(short, long)]
    pub jira: Option<Option<String>>,
    #[arg(short, long)]
    pub git: Option<Option<String>>,
    #[arg(short, long)]
    pub workspace: Option<Option<String>>,
    #[arg(short, long)]
    pub ide: Option<Option<String>>,
}

impl ConfigArgs {

    pub fn is_empty(&self) -> bool {
        self.jira.is_none() && self.git.is_none() && self.workspace.is_none() && self.ide.is_none()
    }
}