use std::path::Path;

use anyhow::Result;

use tempro::cli::CheckArgs;
use tempro::file;
use tempro::template::Template;

pub fn handle_check_command(home: &Path, args: &CheckArgs) -> Result<bool> {
    match &args.name {
        Some(name) => Ok(check_single_template(&home.join(name))),
        None => check_all_templates(home),
    }
}

fn check_single_template(path: &Path) -> bool {
    match Template::read_from_path(&path) {
        Ok(template) => {
            println!("[Passed] {}", template.name());
            true
        }
        Err(e) => {
            eprintln!("[Failed] Error reading template {}: {}", path.display(), e);
            false
        }
    }
}

fn check_all_templates(home: &Path) -> Result<bool> {
    let mut all_passed = true;

    for path in file::get_all_template_paths(home)? {
        let passed = check_single_template(&path);
        if !passed {
            all_passed = false;
        }
    }

    Ok(all_passed)
}
