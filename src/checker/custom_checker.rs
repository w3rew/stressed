use crate::checker::errors::CustomCheckerError;
use crate::checker::{Check, CheckerError};
use crate::communicator::Communicator;
use crate::utils::TestCase;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct CustomChecker {
    checker: Communicator,
}

impl CustomChecker {
    pub fn new(checker: PathBuf) -> CustomChecker {
        CustomChecker {
            checker: Communicator::new(checker, false),
        }
    }
}

impl<T> From<T> for CustomChecker
where
    PathBuf: From<T>,
{
    fn from(val: T) -> CustomChecker {
        CustomChecker::new(PathBuf::from(val))
    }
}

#[async_trait]
impl Check for CustomChecker {
    async fn check(&self, case: &TestCase, answer: &str) -> Result<(), CheckerError> {
        let combined_input = format!("{}{}", case.body, answer);
        let checker_answer = self
            .checker
            .communicate_result(Some(&combined_input), None)
            .await;

        match checker_answer {
            Ok(_) => Ok(()),
            Err(display) => {
                let err = CustomCheckerError::new(display, answer.to_string());
                Err(CheckerError::WrongAnswer(Box::new(err)))
            }
        }
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn custom_checker_fails() {
        let failing_checker = CustomChecker::new(PathBuf::from("false"));

        let tc = TestCase::new(0, "".to_string());

        assert!(matches!(failing_checker.check(&tc, "").await, Err(_)));
    }
    #[tokio::test]
    async fn custom_checker_succeeds() {
        let succeeding_checker = CustomChecker::new(PathBuf::from("true"));

        let tc = TestCase::new(0, "".to_string());

        assert!(matches!(succeeding_checker.check(&tc, "").await, Ok(_)));
    }
}
