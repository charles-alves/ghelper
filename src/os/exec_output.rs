use std::process::Output;

pub enum ExecStatus {
    Success,
    Failure,
}

pub struct ExecOutput {
    output: String,
    outerr: Option<String>,
    status: ExecStatus
}

impl From<Output> for ExecOutput {
    fn from(value: Output) -> Self {
        let mut status= ExecStatus::Failure;
        let mut outerr = None;
        if value.status.success() {
            status = ExecStatus::Success
        } else {
            outerr = Some(String::from_utf8(value.stderr).unwrap());
        };

        Self {
            status,
            outerr,
            output: String::from_utf8(value.stdout).expect("Não foi possível ler o valor do output da execução")
        }
    }
}