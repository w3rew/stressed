use crate::checker::Check;
use crate::communicator::Communicator;
use crate::utils::{TestCase, CompareError};
use std::result::Result;
use similar::{ChangeTag, TextDiff};
use colored::Colorize;
use std::fmt::Display;
use std::path::PathBuf;

use async_trait::async_trait;

pub struct DefaultChecker {
    reference_solver: Communicator,
    line_diff: bool
}

impl DefaultChecker {
    pub fn new(reference_solver: PathBuf, line_diff: bool) -> DefaultChecker {
        DefaultChecker{reference_solver: Communicator::new(reference_solver),
                       line_diff}
    }
}

#[async_trait]
impl Check for DefaultChecker {
    async fn check(&self, testcase: &TestCase, answer: &str) -> Result<(), Box<dyn Display>> {
        let correct_answer = self.reference_solver.communicate(Some(&testcase.body), None).await;

        if correct_answer == answer {
            Ok(())
        } else {
            build_error(testcase, &correct_answer, answer, self.line_diff)
        }
    }
}

fn build_error(testcase: &TestCase,
               correct_answer: &str,
               my_answer: &str,
               line_diff: bool) -> Result<(), Box<dyn Display>> {
    let mut ans = Vec::new();

    let diff = if line_diff {
        TextDiff::from_lines(correct_answer, my_answer)
    } else {
        TextDiff::from_chars(correct_answer, my_answer)
    };

    let mut seen_change = false;

    for change in diff.iter_all_changes() {
        let token = match change.tag() {
            ChangeTag::Delete => {seen_change = true;
                                   change.as_str().unwrap().red()}
            ChangeTag::Insert => {seen_change = true;
                                  change.as_str().unwrap().green()}
            ChangeTag::Equal => change.as_str().unwrap().dimmed()
        };
        ans.push(token);
    }

    if !seen_change {
        unreachable!("Shouldn't have called this function");
    }
    let log = CompareError::new(testcase.clone(), String::from(correct_answer), ans);
    Result::Err(Box::new(log))
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_solution() {
        let checker = DefaultChecker::new(PathBuf::from("cat"), false);

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
        let checker = DefaultChecker::new(PathBuf::from("cat"), false);
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
        let checker = DefaultChecker::new(PathBuf::from("cat"), false);

        for i in 1..100 {
            let testcase = TestCase::new(i, format!("{}\n",  i.to_string()));

            let my_ans = format!("{}\n", i - 1);
            let checked = checker.check(&testcase, &my_ans).await;

            assert!(matches!(checked, Err(_)));
        }
    }
}
