use std::fmt::Debug;
use std::path::Path;

use googletest::{
    description::Description,
    matcher::{Matcher, MatcherBase, MatcherResult},
};

pub fn dir_exist() -> DirExistMatcher {
    DirExistMatcher
}

#[derive(MatcherBase)]
pub struct DirExistMatcher;

// TODO: more detailed explaination
impl<T: AsRef<Path> + Debug + Copy> Matcher<T> for DirExistMatcher {
    fn matches(&self, actual: T) -> MatcherResult {
        actual.as_ref().is_dir().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => "is an existing directory".into(),
            MatcherResult::NoMatch => "isn't an existing directory".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use googletest::matcher::MatcherResult;
    use tempfile::NamedTempFile;

    use crate::test_utils::prelude::*;

    #[test]
    fn not_match_nonexistent_path() -> Result<()> {
        let matcher = dir_exist();
        let result = matcher.matches("/invalid/path");
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn not_match_file() -> Result<()> {
        let matcher = dir_exist();
        let temp_file = NamedTempFile::new()?;
        let result = matcher.matches(temp_file.path());
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn match_directory() -> Result<()> {
        let matcher = dir_exist();
        let temp_dir = tempfile::tempdir()?;
        let result = matcher.matches(temp_dir.path());
        verify_that!(result, eq(MatcherResult::Match))
    }
}
