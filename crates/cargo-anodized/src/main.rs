use clap::{Parser, Subcommand};
use std::{
    collections::BTreeSet as Set,
    path::PathBuf,
    process::{Command, ExitStatus},
};

#[derive(Parser)]
#[command(
    name = "cargo-anodized",
    version,
    about = "Cargo tool integration for Anodized"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format a Cargo project.
    Fmt,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fmt => fmt()?,
    }

    Ok(())
}

fn fmt() -> Result<()> {
    let output = Command::new("cargo")
        .args(["fmt", "--", "--verbose"])
        .output()?;

    let paths = formatted_paths(&output.stdout)?;
    for path in paths {
        println!("{}", path.display());
    }

    if !output.status.success() {
        return Err(Error::CargoFmtFailed(output.status));
    }

    Ok(())
}

fn formatted_paths(output: &[u8]) -> Result<Set<PathBuf>> {
    let output = std::str::from_utf8(output)?;
    output
        .lines()
        .filter_map(|line| line.strip_prefix("Formatting "))
        .map(PathBuf::from)
        .map(|path| path.canonicalize().map_err(Error::from))
        .collect()
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to run cargo fmt: {0}")]
    Io(#[from] std::io::Error),

    #[error("cargo fmt output was not valid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("cargo fmt failed with status {0}")]
    CargoFmtFailed(ExitStatus),
}

type Result<T> = std::result::Result<T, Error>;
