use crate::utils::SeedType;
use colored::Colorize;
use std::fmt;

#[derive(Clone)]
pub struct TestCase {
    pub seed: SeedType,
    pub body: String,
}

impl TestCase {
    pub fn new(seed: SeedType, body: String) -> TestCase {
        TestCase { seed, body }
    }
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{}{}",
            "Testcase (seed = ".bold(),
            self.seed,
            ")".bold()
        )?;
        write!(f, "{}", &self.body)?;
        Ok(())
    }
}
