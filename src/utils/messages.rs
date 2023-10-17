use crate::utils::SeedType;
use colored::Colorize;
use std::fmt;
use std::path::Path;

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

pub struct TestError {
    case: TestCase,
    message: Box<dyn fmt::Display>,
}

impl TestError {
    pub fn new(case: TestCase, message: Box<dyn fmt::Display>) -> TestError {
        TestError { case, message }
    }

    pub fn save_testcase_to(&self, to: &Path) -> Result<(), std::io::Error> {
        std::fs::write(to, &self.case.body)?;
        Ok(())
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)?;
        Ok(())
    }
}
