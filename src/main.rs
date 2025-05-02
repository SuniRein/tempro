mod cli;
mod file;
mod template;

use clap::Parser;
use cli::{Cli, Command};

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
        Command::List(args) => {
            let template_paths = get_all_template_paths(&path);

            let template_names = template_paths
                .iter()
                .filter_map(|p| p.file_name())
                .filter_map(|s| s.to_str())
                .collect::<Vec<_>>();

            if args.table {
                let max_name_length = template_names.iter().map(|s| s.len()).max().unwrap_or(0);

                println!(
                    "{:<width$} {}",
                    "Name",
                    "Description",
                    width = max_name_length
                );

                for template_path in &template_paths {
                    match template::Template::read_from_path(template_path) {
                        Ok(template) => {
                            println!(
                                "{:<width$} {}",
                                template.name(),
                                template.description(),
                                width = max_name_length
                            );
                        }
                        Err(e) => {
                            eprintln!("Error reading template {}: {}", template_path.display(), e);
                            process::exit(1);
                        }
                    }
                }
            } else {
                println!("{}", template_names.join(" "));
            }
        }

        Command::Check(args) => match &args.name {
            Some(name) => match template::Template::read_from_path(&path.join(name)) {
                Ok(template) => {
                    println!("{} is available", template.name());
                }
                Err(e) => {
                    eprintln!("Error reading template {}: {}", name, e);
                    process::exit(1);
                }
            },
            None => {
                let template_paths = get_all_template_paths(&path);

                for template_path in &template_paths {
                    match template::Template::read_from_path(template_path) {
                        Ok(template) => {
                            println!("[Passed] {}", template.name());
                        }
                        Err(e) => {
                            eprintln!(
                                "[Failed] Error reading template {}: {}",
                                template_path.display(),
                                e
                            );
                            process::exit(1);
                        }
                    }
                }
            }
        },
    }
}
