use std::io::{self, Write};
use std::path::Path;

use anyhow::Result;

use crate::cli::ListArgs;
use crate::file;
use crate::template::Template;

pub fn handle_list_command(home: &Path, args: &ListArgs) -> Result<()> {
    let mut names = file::get_all_template_names(home)?;
    names.sort();

    let mut stdout = io::stdout().lock();

    if args.table {
        print_template_table(&mut stdout, home, &names)?;
    } else {
        print_template_names(&mut stdout, &names)?;
    }

    Ok(())
}

fn print_template_names<W: Write>(writer: &mut W, names: &[String]) -> io::Result<()> {
    writeln!(writer, "{}", names.join(" "))
}

fn print_template_table<W: Write>(writer: &mut W, home: &Path, names: &[String]) -> Result<()> {
    let mut max_name_len = "Name".len();

    for name in names {
        max_name_len = max_name_len.max(name.len());
    }

    writeln!(
        writer,
        "{:<width$} Description",
        "Name",
        width = max_name_len
    )?;

    writeln!(writer, "{:-<width$} -----------", "", width = max_name_len)?; // separator

    if names.is_empty() {
        writeln!(writer, "(no templates found)")?;
        return Ok(());
    }

    for name in names {
        let template = Template::load(&home.join(name))?;
        writeln!(
            writer,
            "{:<width$} {}",
            template.name(),
            template.description(),
            width = max_name_len
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::TemplateHome;

    fn setup() -> TemplateHome {
        let mut home = TemplateHome::new();
        home.push("foo", Some(r#"description = "Foo template""#));
        home.push("baz", Some(r#"description = "Baz template""#));
        home.push("bar", Some(r#"description = "Bar template""#));
        home
    }

    mod test_print_template_name {
        use super::*;

        #[test]
        fn empty_list() {
            let mut output = Vec::new();
            print_template_names(&mut output, &[]).unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(output, "\n");
        }

        #[test]
        fn it_works() {
            let home = setup();

            let mut names = file::get_all_template_names(home.path()).unwrap();
            names.sort();

            let mut output = Vec::new();
            print_template_names(&mut output, &names).unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(output, "bar baz foo\n");
        }
    }

    mod test_print_template_table {
        use super::*;

        #[test]
        fn empty_list() {
            let mut output = Vec::new();
            print_template_table(&mut output, Path::new(""), &[]).unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(
                output,
                "Name Description\n---- -----------\n(no templates found)\n"
            );
        }

        #[test]
        fn short_names() {
            let home = setup();

            let mut names = file::get_all_template_names(home.path()).unwrap();
            names.sort();

            let mut output = Vec::new();
            print_template_table(&mut output, home.path(), &names).unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(
                output,
                "Name Description\n\
                 ---- -----------\n\
                 bar  Bar template\n\
                 baz  Baz template\n\
                 foo  Foo template\n"
            );
        }

        #[test]
        fn longer_names() {
            let mut home = setup();
            home.push(
                "longer_template_name",
                Some(r#"description = "Longer template name""#),
            );

            let mut names = file::get_all_template_names(home.path()).unwrap();
            names.sort();

            let mut output = Vec::new();
            print_template_table(&mut output, home.path(), &names).unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(
                output,
                "Name                 Description\n\
                 -------------------- -----------\n\
                 bar                  Bar template\n\
                 baz                  Baz template\n\
                 foo                  Foo template\n\
                 longer_template_name Longer template name\n"
            );
        }
    }
}
