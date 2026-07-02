use std::fmt::{Display, Formatter};
use clap::Args;

pub enum BranchType {
    MajorRelease,
    Release,
    Hotfix,
    Fix,
    Feature
}

impl Display for BranchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BranchType::MajorRelease => String::from("release!"),
            BranchType::Release => String::from("release"),
            BranchType::Hotfix => String::from("hotfix"),
            BranchType::Fix => String::from("fix"),
            BranchType::Feature => String::from("feature"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Args)]
#[group(multiple = false)]
#[derive(Debug)]
pub struct CheckoutBranchArgs {
    /// Branch para criação de versões Major
    #[arg(short, long, conflicts_with = "native_args")]
    major_release: bool,
    /// Branch para criação de versões Minor
    #[arg(short, long, conflicts_with = "native_args")]
    release: bool,
    /// Branch para criação de versões Patch
    #[arg(short = 't', long, conflicts_with = "native_args")]
    hotfix: bool,
    /// Utilizada para trabalhar em pequenas correções na release
    #[arg(short = 'x', long, conflicts_with = "native_args")]
    fix: bool,
    /// Utilizada para implementação de novas funcionalizades para release
    #[arg(short, long, conflicts_with = "native_args")]
    feature: bool,
}

impl CheckoutBranchArgs {

    pub fn value(&self) -> Option<BranchType> {
        if self.major_release {
            Some(BranchType::MajorRelease)
        } else if self.release {
            Some(BranchType::Release)
        } else if self.hotfix {
            Some(BranchType::Hotfix)
        } else if self.fix {
            Some(BranchType::Fix)
        } else if self.feature {
            Some(BranchType::Feature)
        } else {
            None
        }
    }
}