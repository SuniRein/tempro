mod cli;
mod file;
mod template;

use clap::Parser;
use cli::Cli;

use std::process;

fn main() {
    Cli::parse();

    let path = file::get_template_path().unwrap_or_else(|| {
        eprintln!("Error: Could not determine the template path.");
        process::exit(1);
    });
}
