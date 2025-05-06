use std::io::{self, Write};
use std::path::Path;

use anyhow::{Result, bail};

use crate::cli::CheckArgs;
use crate::file;
use crate::template::Template;

pub fn handle_check_command(home: &Path, args: &CheckArgs) -> Result<()> {
    let names = match &args.name {
        Some(name) => vec![name.clone()],
        None => file::get_all_template_names(home)?,
    };

    let results = check_templates(home, &names);

    #[cfg(not(test))]
    {
        let mut stdout = io::stdout().lock();
        print_check_results(&mut stdout, &results)?;
    }

    if !results.iter().all(|r| r.result.is_ok()) {
        bail!("Some templates failed to load.");
    }

    Ok(())
}

fn print_check_results<W: Write>(writer: &mut W, results: &[CheckResult]) -> io::Result<()> {
    for CheckResult { name, result } in results.iter() {
        match result {
            Ok(_) => writeln!(writer, "[Passed] {name}")?,
            Err(e) => writeln!(writer, "[Failed] {name}: {e}")?,
        }
    }

    Ok(())
}

#[derive(Debug)]
struct CheckResult {
    pub name: String,
    pub result: Result<()>,
}

fn check_templates(home: &Path, names: &[String]) -> Vec<CheckResult> {
    names
        .iter()
        .map(|name| CheckResult {
            name: name.clone(),
            result: Template::load(&home.join(name)).map(|_| ()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::anyhow;
    use hamcrest2::prelude::*;

    use crate::test_utils::TemplateHome;

    fn setup() -> TemplateHome {
        let mut home = TemplateHome::new();
        home.push("Template1", Some(r#"description = "Template1 success""#));
        home.push("Template2", Some(r#"description = "Template2 success""#));
        home.push("Template3", Some(r#"no description"#));
        home.push("Template4", None);
        home.push("Template5", Some(r#"description = "Template5 success""#));
        home
    }

    mod test_check_templates {
        use super::*;

        fn assert_result(results: &[CheckResult], name: &str, passed: bool) {
            let result = results.iter().find(|r| r.name == name).unwrap_or_else(|| {
                panic!("Template {name} not found in results");
            });

            assert_eq!(
                result.result.is_ok(),
                passed,
                "expected {name} to be {passed}"
            );
        }

        #[test]
        fn it_works() {
            let home = setup();

            let names = [
                "Template1".to_string(),
                "Template2".to_string(),
                "Template3".to_string(),
                "Template4".to_string(),
                "Template5".to_string(),
            ];

            let results = check_templates(home.path(), &names);

            assert_eq!(results.len(), 5);
            assert_result(&results, "Template1", true);
            assert_result(&results, "Template2", true);
            assert_result(&results, "Template3", false);
            assert_result(&results, "Template4", false);
            assert_result(&results, "Template5", true);
        }
    }

    #[test]
    fn test_print_check_results() {
        let mut buffer = Vec::new();

        let results = vec![
            CheckResult {
                name: "Template1".to_string(),
                result: Ok(()),
            },
            CheckResult {
                name: "Template2".to_string(),
                result: Err(anyhow!("Failed to load")),
            },
        ];

        print_check_results(&mut buffer, &results).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output,
            "[Passed] Template1\n[Failed] Template2: Failed to load\n",
        );
    }

    mod test_handle_check_command {
        use super::*;

        #[test]
        fn with_name_passed() {
            let home = TemplateHome::single("Test", Some(r#"description = "Test template""#));
            let args = CheckArgs {
                name: Some("Test".to_string()),
            };

            let result = handle_check_command(home.path(), &args);
            assert_that!(&result, ok());
        }

        #[test]
        fn with_name_failed() {
            let home = TemplateHome::single("Test", Some(r#"no description"#));
            let args = CheckArgs {
                name: Some("Test".to_string()),
            };

            let result = handle_check_command(home.path(), &args);
            assert_that!(result, err());
        }

        #[test]
        fn without_name_passed() {
            let mut home = TemplateHome::new();
            home.push("Template1", Some(r#"description = "Template1 success""#));
            home.push("Template2", Some(r#"description = "Template2 success""#));

            let args = CheckArgs { name: None };

            let result = handle_check_command(home.path(), &args);
            assert_that!(result, ok());
        }

        #[test]
        fn without_name_failed() {
            let home = setup();
            let args = CheckArgs { name: None };

            let result = handle_check_command(home.path(), &args);
            assert_that!(result, err());
        }
    }
}
