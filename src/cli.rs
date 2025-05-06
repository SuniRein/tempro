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

    /// Check if a template is available
    Check(CheckArgs),

    /// Apply a template in the given directory
    Apply(ApplyArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Show template information in a table
    #[arg(short, long)]
    pub table: bool,
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// The name of the template to check
    /// (leave empty to check all templates)
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct ApplyArgs {
    /// The name of the template to apply
    pub name: String,

    /// The directory to apply the template in
    pub target: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
