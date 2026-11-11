mod command;

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Format a Cargo project.
    Fmt(command::fmt::Options),
    /// Verify a Cargo project with Aeneas and Lean.
    VerifyAeneas(command::verify_aeneas::Options),
    #[cfg(feature = "z3")]
    /// Emulates `z3 -smt2 -in`.
    Z3Smt2In(command::z3::Options),
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let cli: Cli = clap::Parser::parse();

    match cli.command {
        Command::Fmt(options) => command::fmt::fmt(options)?,
        Command::VerifyAeneas(options) => command::verify_aeneas::run(options)?,
        #[cfg(feature = "z3")]
        Command::Z3Smt2In(options) => command::z3::emulate_smt2_in(options)?,
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Fmt(#[from] command::fmt::Error),

    #[error(transparent)]
    VerifyAeneas(#[from] command::verify_aeneas::Error),

    #[cfg(feature = "z3")]
    #[error(transparent)]
    Z3Smt2In(#[from] command::z3::Error),
}
