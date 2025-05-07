use std::fmt::Debug;
use std::fs;
use std::path::Path;

use googletest::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};

pub fn file_content<T: AsRef<str>>(expected: T) -> FileContentMatcher {
    FileContentMatcher {
        expected: expected.as_ref().to_string(),
    }
}

#[derive(MatcherBase)]
pub struct FileContentMatcher {
    expected: String,
}

impl<T: AsRef<Path> + Debug + Copy> Matcher<T> for FileContentMatcher {
    fn matches(&self, actual: T) -> MatcherResult {
        let actual_content = fs::read_to_string(actual.as_ref()).unwrap_or_default();
        (self.expected == actual_content).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("has file content `{}`", self.expected).into(),
            MatcherResult::NoMatch => {
                format!("doesn't have file content `{}`", self.expected).into()
            }
        }
    }

    fn explain_match(&self, actual: T) -> Description {
        let actual_content = fs::read_to_string(actual.as_ref());
        match actual_content {
            Ok(content) => format!(
                "expected file content `{}`, but got `{}`",
                self.expected, content
            )
            .into(),
            Err(err) => format!("failed to read file: {err}").into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use googletest::matcher::MatcherResult;
    use tempfile::NamedTempFile;

    use crate::test_utils::prelude::*;

    #[test]
    fn match_file_with_content() -> Result<()> {
        let matcher = file("Test content");

        let temp_file = NamedTempFile::new()?;
        temp_file.as_file().write_all(b"Test content")?;

        let result = matcher.matches(temp_file.path());
        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn not_match_nonexistent_path() -> Result<()> {
        let matcher = file("Test content");

        let result = matcher.matches("/invalid/path/to/file");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn not_match_non_file() -> Result<()> {
        let matcher = file("Test content");

        let temp_dir = tempfile::tempdir()?;

        let result = matcher.matches(temp_dir.path());
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn not_match_file_with_different_content() -> Result<()> {
        let matcher = file("Test content");

        let temp_file = NamedTempFile::new()?;
        temp_file.as_file().write_all(b"Different content")?;

        let result = matcher.matches(temp_file.path());
        verify_that!(result, eq(MatcherResult::NoMatch))
    }
}
