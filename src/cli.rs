use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List all available templates
    List(ListArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show template information in a table
    #[arg(short, long)]
    pub table: bool,
}
