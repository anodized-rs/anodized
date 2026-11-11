use std::process::ExitStatus;

use crate::cli::VerifyAeneasOptions;

type Result<T> = std::result::Result<T, Error>;

pub fn run(options: VerifyAeneasOptions) -> Result<()> {
    let VerifyAeneasOptions {
        packages,
        manifest_path,
        verbose,
    } = options;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`cargo fmt` failed with status {0}")]
    CommandFailed(ExitStatus),
}
