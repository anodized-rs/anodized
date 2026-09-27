use clap::Parser;

use crate::cli::{Cli, Command};

mod cli;
mod commands;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Fmt(options) => commands::fmt::fmt(options)?,
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("command `fmt` failed: {0}")]
    Fmt(#[from] commands::fmt::Error),
}
