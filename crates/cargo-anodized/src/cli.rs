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
    Fmt,
}
