use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Format a Cargo project.
    Fmt(crate::command::fmt::Options),
    /// Verify a Cargo project with Aeneas and Lean.
    VerifyAeneas(crate::command::verify_aeneas::Options),
    #[cfg(feature = "z3")]
    /// Emulates `z3 -smt2 -in`.
    Z3Smt2In(crate::command::z3::Options),
}
