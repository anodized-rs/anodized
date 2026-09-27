use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cargo-anodized",
    version,
    about = "Cargo tool integration for Anodized"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Format a Cargo project.
    Fmt {
        /// Specify packages to format.
        #[arg(short, long = "package", value_name = "PACKAGE")]
        packages: Vec<String>,

        /// Specify the path to Cargo.toml.
        #[arg(long, value_name = "PATH")]
        manifest_path: Option<PathBuf>,

        /// Format all packages in the workspace.
        #[arg(long)]
        all: bool,

        /// Check formatting without modifying files.
        #[arg(long)]
        check: bool,
    },
}
