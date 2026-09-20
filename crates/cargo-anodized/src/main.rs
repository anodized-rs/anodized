use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fmt => fmt(),
    }
}

fn fmt() {
    todo!()
}
