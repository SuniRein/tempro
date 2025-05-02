use std::path::{Path, PathBuf};

use anyhow::Result;

use tempro::cli::ListArgs;
use tempro::file;
use tempro::template::Template;

pub fn handle_list_command(home: &Path, args: &ListArgs) -> Result<()> {
    let paths = file::get_all_template_paths(home)?;

    match args.table {
        true => print_template_table(&paths)?,
        false => print_template_names(&paths),
    }

    Ok(())
}

fn print_template_names(paths: &[PathBuf]) {
    let names = paths
        .iter()
        .filter_map(|p| p.file_name())
        .filter_map(|s| s.to_str())
        .map(String::from)
        .collect::<Vec<_>>();

    println!("{}", names.join(" "));
}

fn print_template_table(paths: &[PathBuf]) -> Result<()> {
    let templates = paths
        .iter()
        .map(|path| Template::read_from_path(path))
        .collect::<Result<Vec<_>>>()?;

    let max_name_length = templates.iter().map(|t| t.name().len()).max().unwrap_or(4);

    println!(
        "{:<width$} {}",
        "Name",
        "Description",
        width = max_name_length
    );

    for template in templates {
        println!(
            "{:<width$} {}",
            template.name(),
            template.description(),
            width = max_name_length
        );
    }

    Ok(())
}
