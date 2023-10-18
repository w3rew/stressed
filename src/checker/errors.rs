use crate::utils::DELIMITER_STR;
use colored::{ColoredString, Colorize};
use std::error::Error;
use std::fmt;
type DiffItem = Vec<ColoredString>;

pub enum CheckerError {
    RuntimeError(Box<dyn Error>),
    WrongAnswer(Box<dyn Error>),
}

#[derive(Debug)]
pub struct CompareError {
    pub correct_answer: String,
    pub my_answer: DiffItem,
}

impl CompareError {
    pub fn new(correct_answer: String, my_answer: DiffItem) -> CompareError {
        CompareError {
            correct_answer,
            my_answer,
        }
    }
}

impl fmt::Display for CompareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl Error for CompareError {}

#[derive(Debug)]
pub struct CustomCheckerError {
    pub checker_output: Box<dyn Error>,
    pub my_output: String,
}

impl CustomCheckerError {
    pub fn new(checker_output: Box<dyn Error>, my_output: String) -> CustomCheckerError {
        CustomCheckerError {
            checker_output,
            my_output,
        }
    }
}

impl fmt::Display for CustomCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "Checker output:".bold())?;
        write!(f, "{}", &self.checker_output)?;
        writeln!(f, "{DELIMITER_STR}")?;
        writeln!(f, "{}", "Program output:".bold())?;
        write!(f, "{}", self.my_output)?;

        Ok(())
    }
}
impl Error for CustomCheckerError {}
