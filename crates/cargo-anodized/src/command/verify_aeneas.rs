use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Options {
    /// Specify packages to format.
    #[arg(short, long = "package", value_name = "PACKAGE")]
    pub packages: Vec<String>,

    /// Specify the path to Cargo.toml.
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<PathBuf>,

    /// Use verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

type Result<T> = std::result::Result<T, Error>;

pub fn run(options: Options) -> Result<()> {
    let Options {
        packages,
        manifest_path,
        verbose,
    } = options;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`cargo fmt` failed with status {0}")]
    CommandFailed(std::process::ExitStatus),
}
