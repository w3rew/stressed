use crate::args::DiffMode;
use crate::checker::errors::CompareError;
use crate::checker::Check;
use crate::communicator::Communicator;
use crate::utils::TestCase;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::fmt::Display;
use std::path::PathBuf;
use std::result::Result;

use async_trait::async_trait;

pub struct DefaultChecker {
    reference_solver: Communicator,
    diff_mode: DiffMode,
}

impl DefaultChecker {
    pub fn new(reference_solver: PathBuf, diff_mode: DiffMode) -> DefaultChecker {
        DefaultChecker {
            reference_solver: Communicator::new(reference_solver),
            diff_mode,
        }
    }
}

#[async_trait]
impl Check for DefaultChecker {
    async fn check(&self, testcase: &TestCase, answer: &str) -> Result<(), Box<dyn Display>> {
        let correct_answer = self
            .reference_solver
            .communicate(Some(&testcase.body), None)
            .await;

        if correct_answer == answer {
            Ok(())
        } else {
            build_error(testcase, &correct_answer, answer, self.diff_mode)
        }
    }
}

fn build_error(
    testcase: &TestCase,
    correct_answer: &str,
    my_answer: &str,
    diff_mode: DiffMode,
) -> Result<(), Box<dyn Display>> {
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
    let log = CompareError::new(testcase.clone(), String::from(correct_answer), ans);
    Result::Err(Box::new(log))
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_solution() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None);

        let my_prog = Communicator::new(PathBuf::from("cat"));

        for i in 0..100 {
            let testcase = TestCase::new(i, i.to_string());

            let my_ans = my_prog.communicate(Some(&testcase.body), None).await;
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Ok(())));
        }
    }

    #[tokio::test]
    async fn no_newline() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None);
        let my_prog = Communicator::new(PathBuf::from("cat"));

        for i in 0..100 {
            let testcase = TestCase::new(i, format!("{i}\n"));

            let my_ans = my_prog.communicate(Some(&testcase.body), None).await;
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Ok(())));
        }
    }

    #[tokio::test]
    async fn minus_one() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), DiffMode::None);

        for i in 1..100 {
            let testcase = TestCase::new(i, format!("{}\n", i.to_string()));

            let my_ans = format!("{}\n", i - 1);
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Err(_)));
        }
    }
}
