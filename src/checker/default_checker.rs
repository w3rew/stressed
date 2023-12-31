use crate::args::DiffMode;
use crate::checker::errors::CompareError;
use crate::checker::{Check, CheckerError};
use crate::communicator::Communicator;
use crate::utils::TestCase;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;
use std::result::Result;

use async_trait::async_trait;

pub struct DefaultChecker {
    reference_solver: Communicator,
    diff_mode: DiffMode,
}

impl DefaultChecker {
    pub fn new(
        reference_solver: PathBuf,
        diff_mode: DiffMode,
        trim_output: bool,
    ) -> DefaultChecker {
        DefaultChecker {
            reference_solver: Communicator::new(reference_solver, trim_output),
            diff_mode,
        }
    }
}

#[async_trait]
impl Check for DefaultChecker {
    async fn check(&self, testcase: &TestCase, answer: &str) -> Result<(), CheckerError> {
        let correct_answer = self
            .reference_solver
            .communicate_result(Some(&testcase.body), None)
            .await;
        let correct_answer = match correct_answer {
            Err(e) => return Err(CheckerError::RuntimeError(e)),
            Ok(x) => x,
        };
        if correct_answer == answer {
            Ok(())
        } else {
            Err(CheckerError::WrongAnswer(Box::new(build_error(
                &correct_answer,
                answer,
                self.diff_mode,
            ))))
        }
    }
}

fn build_error(correct_answer: &str, my_answer: &str, diff_mode: DiffMode) -> CompareError {
    let mut ans = Vec::new();

    if let DiffMode::None = diff_mode {
        ans.push(my_answer.normal());
    } else {
        let diff = match diff_mode {
            DiffMode::Line => TextDiff::from_lines(correct_answer, my_answer),
            DiffMode::Char => TextDiff::from_chars(correct_answer, my_answer),
            _ => unreachable!(),
        };

        let mut seen_change = false;

        for change in diff.iter_all_changes() {
            let token = match change.tag() {
                ChangeTag::Delete => {
                    seen_change = true;
                    change.as_str().unwrap().red()
                }
                ChangeTag::Insert => {
                    seen_change = true;
                    change.as_str().unwrap().green()
                }
                ChangeTag::Equal => change.as_str().unwrap().dimmed(),
            };
            ans.push(token);
        }

        if !seen_change {
            unreachable!("Shouldn't have called this function");
        }
    }
    CompareError::new(String::from(correct_answer), ans)
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_solution() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None, false);

        let my_prog = Communicator::new(PathBuf::from("cat"), false);

        for i in 0..100 {
            let testcase = TestCase::new(i, i.to_string());

            let my_ans = my_prog.communicate(Some(&testcase.body), None).await;
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Ok(())));
        }
    }

    #[tokio::test]
    async fn no_newline() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None, false);
        let my_prog = Communicator::new(PathBuf::from("cat"), false);

        for i in 0..100 {
            let testcase = TestCase::new(i, format!("{i}\n"));

            let my_ans = my_prog.communicate(Some(&testcase.body), None).await;
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Ok(())));
        }
    }

    #[tokio::test]
    async fn minus_one() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None, false);

        for i in 1..100 {
            let testcase = TestCase::new(i, format!("{}\n", i.to_string()));

            let my_ans = format!("{}\n", i - 1);
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Err(_)));
        }
    }
}
