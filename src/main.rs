use anyhow::{Context, Result};
use clap::Parser;

use tempro::cli::{Cli, Command};
use tempro::command;
use tempro::file;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let home = file::get_template_home().context("failed to get template home")?;

    match &cli.command {
        Command::List(args) => command::handle_list_command(&home, args),
        Command::Check(args) => command::handle_check_command(&home, args),
        Command::Apply(args) => command::handle_apply_command(&home, args),
    }
}
