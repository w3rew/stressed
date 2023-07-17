mod default_checker;
mod custom_checker;

use crate::utils::TestCase;
use std::result::Result;
use std::fmt;

pub use default_checker::DefaultChecker;
pub use custom_checker::CustomChecker;

pub trait Check {
    fn check(&self, input: &TestCase, answer: &str) -> Result<(), Box<dyn fmt::Display>>;
}
