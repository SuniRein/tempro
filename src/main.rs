mod cli;
mod file;
mod template;

use clap::Parser;
use cli::Cli;

use std::path::PathBuf;
use std::process;

fn get_all_template_paths(path: &PathBuf) -> Vec<PathBuf> {
    path.read_dir()
        .unwrap_or_else(|_| {
            eprintln!("Error: Could not read the template directory.");
            process::exit(1);
        })
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() { Some(path) } else { None }
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();

    let path = file::get_template_path().unwrap_or_else(|| {
        eprintln!("Error: Could not determine the template path.");
        process::exit(1);
    });

    match &cli.command {
        cli::Command::List => {
            let template_paths = get_all_template_paths(&path);
            let template_names = template_paths
                .iter()
                .filter_map(|p| p.file_name())
                .filter_map(|s| s.to_str())
                .collect::<Vec<_>>()
                .join(" ");

            println!("{template_names}");
        }
    }
}
