use std::process::Output;

use crate::os::exec_output::ExecOutput::Failure;

pub enum ExecOutput {
    Success(String),
    Failure(String),
}

impl From<Output> for ExecOutput {
    fn from(value: Output) -> Self {
        if value.status.success() {
            Self::Success(String::from_utf8(value.stdout).unwrap())
        } else {
            Failure(String::from_utf8(value.stderr).unwrap())
        }
    }
}