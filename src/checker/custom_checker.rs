use crate::checker::Check;
use std::fmt;
use crate::utils::TestCase;
use std::path::PathBuf;

pub struct CustomChecker {
    checker: PathBuf,
}

impl CustomChecker {
    pub fn new(checker: PathBuf) -> CustomChecker {
        CustomChecker{checker}
    }
}

impl<T> From<T> for CustomChecker where PathBuf: From<T> {
    fn from(val: T) -> CustomChecker {
        CustomChecker::new(PathBuf::from(val))
    }
}

impl Check for CustomChecker {
    fn check(&self, input: &TestCase, answer: &str) -> Result<(), Box<dyn fmt::Display>> {
        unreachable!("Unimplemented");
    }
}
