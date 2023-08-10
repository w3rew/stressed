mod default_checker;
mod custom_checker;

use crate::utils::TestCase;
use std::result::Result;
use std::fmt;

use async_trait::async_trait;

pub use default_checker::DefaultChecker;
pub use custom_checker::CustomChecker;

#[async_trait]
pub trait Check {
    async fn check(&self, input: &TestCase, answer: &str) -> Result<(), Box<dyn fmt::Display>>;
}

pub trait Checker: Sync + Check {}
impl<T> Checker for T where T: Sync + Check {}
