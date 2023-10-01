use crate::utils::TestCase;
use colored::{ColoredString, Colorize};
use std::fmt;
type DiffItem = Vec<ColoredString>;
const DELIMITER_STR: &'static str = "--------------------"; //20 symbols

pub struct CompareError {
    pub testcase: TestCase,
    pub correct_answer: String,
    pub my_answer: DiffItem,
}

impl CompareError {
    pub fn new(testcase: TestCase, correct_answer: String, my_answer: DiffItem) -> CompareError {
        CompareError {
            testcase,
            correct_answer,
            my_answer,
        }
    }
}

impl fmt::Display for CompareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "ERROR".red())?;
        writeln!(f, "{DELIMITER_STR}")?;

        write!(f, "{}", self.testcase)?;

        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Correct answer:".bold())?;
        write!(f, "{}", &self.correct_answer)?;
        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Program answer:".bold())?;

        for text in self.my_answer.iter() {
            write!(f, "{}", text)?;
        }

        Ok(())
    }
}

pub struct CustomCheckerError {
    pub testcase: TestCase,
    pub checker_output: String,
    pub my_output: String,
}

impl CustomCheckerError {
    pub fn new(
        testcase: TestCase,
        checker_output: String,
        my_output: String,
    ) -> CustomCheckerError {
        CustomCheckerError {
            testcase,
            checker_output,
            my_output,
        }
    }
}

impl fmt::Display for CustomCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "ERROR".red())?;
        writeln!(f, "{DELIMITER_STR}")?;

        write!(f, "{}", self.testcase)?;

        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Checker output:".bold())?;
        write!(f, "{}", &self.checker_output)?;
        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Program output:".bold())?;
        write!(f, "{}", self.my_output)?;

        Ok(())
    }
}
