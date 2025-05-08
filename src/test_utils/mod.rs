mod matchers;
mod template_home;

pub mod temp_wd;

pub use template_home::TemplateHome;

pub mod prelude {
    pub use googletest::prelude::*;

    pub use crate::test_utils::matchers::*;
}
