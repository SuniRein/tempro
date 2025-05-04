use std::path::Path;

use anyhow::Result;

use tempro::cli::CheckArgs;
use tempro::file;
use tempro::template::Template;

pub fn handle_check_command(home: &Path, args: &CheckArgs) -> Result<bool> {
    match &args.name {
        Some(name) => Ok(check_single_template(home, name)),
        None => check_all_templates(home),
    }
}

fn check_single_template(home: &Path, name: &str) -> bool {
    match Template::read_from_path(&home.join(name)) {
        Ok(_) => {
            println!("[Passed] {name}");
            true
        }
        Err(e) => {
            eprintln!("[Failed] {name}: {e}");
            false
        }
    }
}

fn check_all_templates(home: &Path) -> Result<bool> {
    let mut all_passed = true;

    for name in file::get_all_template_names(home)? {
        let passed = check_single_template(home, &name);
        if !passed {
            all_passed = false;
        }
    }

    Ok(all_passed)
}
