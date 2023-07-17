use colored::{Colorize, ColoredString};
use std::fmt;
use crate::utils::TestCase;
type DiffItem = Vec<ColoredString>;

const DELIMITER_STR: &'static str = "--------------------"; //20 symbols

pub struct CompareError {
    pub testcase: TestCase,
    pub correct_answer: String,
    pub my_answer: DiffItem,
}

impl CompareError {
    pub fn new(testcase: TestCase, correct_answer: String, my_answer: DiffItem) -> CompareError {
        CompareError{testcase, correct_answer, my_answer}
    }
}

impl fmt::Display for CompareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "ERROR".red())?;
        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}{}{}", "Testcase (seed = ".bold(), self.testcase.seed, ")".bold())?;
        write!(f, "{}", &self.testcase.body)?;
        writeln!(f, "{DELIMITER_STR}")?;
        write!(f, "{}", "Correct answer:".bold())?;
        writeln!(f, "{}", &self.correct_answer)?;
        write!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Given answer:".bold())?;

        for text in self.my_answer.iter() {
            write!(f, "{}", text)?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}
