use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod commands;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fmt => commands::fmt::fmt()?,
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("command `fmt` failed: {0}")]
    Fmt(#[from] commands::fmt::Error),
}
