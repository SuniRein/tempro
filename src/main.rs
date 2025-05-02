mod command;

use std::process;

use anyhow::{Result, anyhow};
use clap::Parser;

use tempro::cli::{Cli, Command};
use tempro::file;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let home = file::get_template_home().ok_or_else(|| anyhow!("failed to get template home"))?;

    match &cli.command {
        Command::List(args) => command::handle_list_command(&home, args),
        Command::Check(args) => {
            if !command::handle_check_command(&home, args)? {
                process::exit(1);
            }
            process::exit(0);
        }
    }
}
