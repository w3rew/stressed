use crate::utils::SeedType;
use colored::Colorize;
use std::error::Error;
use std::fmt;
use std::path::Path;

const RUNTIME_ERROR_MSG: &'static str = "<RUNTIME ERROR>";
const SAMPLER_ERROR_MSG: &'static str = "<SAMPLER ERROR>";
const CHECKER_ERROR_MSG: &'static str = "<CHECKER ERROR>";
pub const DELIMITER_STR: &'static str = "--------------------"; //20 symbols

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct ProgramFailure(String);

impl fmt::Display for ProgramFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ProgramFailure {
    pub fn new(s: String) -> ProgramFailure {
        ProgramFailure(s)
    }
}

impl Error for ProgramFailure {}

#[derive(Debug)]
pub enum TestResult {
    SamplerError(Box<dyn Error>),
    CheckerError {
        testcase: TestCase,
        err: Box<dyn Error>,
    },
    SolutionRuntimeError {
        testcase: TestCase,
        err: Box<dyn Error>,
    },
    WrongAnswer {
        testcase: TestCase,
        msg: Box<dyn Error>,
    },
    Ok,
}

impl TestResult {
    pub fn save_testcase_to(&self, to: &Path) -> Result<(), std::io::Error> {
        match self {
            TestResult::CheckerError { testcase, .. }
            | TestResult::SolutionRuntimeError { testcase, .. }
            | TestResult::WrongAnswer { testcase, .. } => {
                std::fs::write(to, &testcase.body)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "ERROR".red())?;
        writeln!(f, "{DELIMITER_STR}")?;
        match self {
            TestResult::SamplerError(ref u) => {
                writeln!(f, "{}", SAMPLER_ERROR_MSG.red())?;
                write!(f, "{u}")?;
            }
            TestResult::CheckerError { testcase, err }
            | TestResult::SolutionRuntimeError { testcase, err } => {
                write!(f, "{testcase}")?;
                writeln!(f, "{DELIMITER_STR}")?;
                if let TestResult::CheckerError {
                    testcase: _,
                    err: _,
                } = self
                {
                    writeln!(f, "{}", CHECKER_ERROR_MSG.red())?;
                } else {
                    writeln!(f, "{}", RUNTIME_ERROR_MSG.red())?;
                }
                write!(f, "{err}")?;
            }
            TestResult::WrongAnswer { testcase, msg } => {
                write!(f, "{testcase}")?;
                writeln!(f, "{DELIMITER_STR}")?;
                write!(f, "{msg}")?;
            }
            TestResult::Ok => {}
        }
        Ok(())
    }
}

impl Error for TestResult {}
