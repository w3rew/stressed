use crate::checker::Check;
use crate::solver::Solver;
use crate::utils::{TestCase, CompareError};
use std::result::Result;
use similar::{ChangeTag, TextDiff};
use colored::{ColoredString, Colorize};
use std::fmt::Display;

pub struct DefaultChecker {
    reference_solver: Solver,
}

impl DefaultChecker {
    pub fn from_solver(reference_solver: Solver) -> DefaultChecker {
        DefaultChecker{reference_solver}
    }

    pub fn new(reference_executable: &str) -> DefaultChecker {
        DefaultChecker::from_solver(Solver::new(reference_executable))
    }
}

impl Check for DefaultChecker {
    fn check(&self, testcase: &TestCase, answer: &str) -> Result<(), Box<dyn Display>> {
        let correct_answer = self.reference_solver.interact(&testcase.body);

        build_error(testcase, &correct_answer, answer)
    }
}

fn build_error(testcase: &TestCase,
               correct_answer: &str,
               my_answer: &str) -> Result<(), Box<dyn Display>> {
    let mut ans = Vec::new();

    let diff = TextDiff::from_chars(correct_answer, my_answer);

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
        return Result::Ok(());
    }
    let log = CompareError::new(testcase.clone(), String::from(correct_answer), ans);
    Result::Err(Box::new(log))
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[test]
    fn echo_solution() {
        let checker = DefaultChecker::new("cat");

        let my_prog = Solver::new("cat");

        for i in 0..100 {
            let testcase = TestCase::new(i, i.to_string());

            let my_ans = my_prog.interact(&testcase.body);
            let checked = checker.check(&testcase, &my_ans);

            assert!(matches!(checked, Ok(())));
        }
    }

    #[test]
    fn no_newline() {
        let checker = DefaultChecker::new("cat");

        let my_prog = Solver::new("cat");

        for i in 0..100 {
            let testcase = TestCase::new(i, format!("{i}\n"));

            let mut my_ans = my_prog.interact(&testcase.body);
            let checked = checker.check(&testcase, &my_ans);

            assert!(matches!(checked, Ok(())));
        }
    }

    #[test]
    fn minus_one() {
        let checker = DefaultChecker::new("cat");

        let my_prog = Solver::new("cat");

        for i in 1..100 {
            let testcase = TestCase::new(i, format!("{}\n",  i.to_string()));

            let mut my_ans = format!("{}\n", i - 1);
            let checked = checker.check(&testcase, &my_ans);

            assert!(matches!(checked, Err(_)));
        }
    }
}
