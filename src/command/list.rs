use std::path::Path;

use anyhow::Result;

use crate::cli::ListArgs;
use crate::file;
use crate::template::Template;

pub fn handle_list_command(home: &Path, args: &ListArgs) -> Result<()> {
    let mut names = file::get_all_template_names(home)?;
    names.sort();

    match args.table {
        true => print_template_table(home, &names)?,
        false => print_template_names(&names),
    }

    Ok(())
}

fn print_template_names(names: &[String]) {
    println!("{}", names.join(" "));
}

fn print_template_table(home: &Path, names: &[String]) -> Result<()> {
    let templates = names
        .iter()
        .map(|name| Template::read_from_path(&home.join(name)))
        .collect::<Result<Vec<_>>>()?;

    let max_name_length = names.iter().map(|name| name.len()).max().unwrap_or(4);

    println!("{:<width$} Description", "Name", width = max_name_length);

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
