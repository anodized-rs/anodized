use clap::Parser;

#[derive(Parser)]
#[command(
    name = "cargo-anodized",
    version,
    about = "Cargo tool integration for Anodized"
)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
