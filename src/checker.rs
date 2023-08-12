mod custom_checker;
mod default_checker;

use crate::utils::TestCase;
use std::fmt;
use std::result::Result;

use async_trait::async_trait;

pub use custom_checker::CustomChecker;
pub use default_checker::DefaultChecker;

#[async_trait]
pub trait Check {
    async fn check(&self, input: &TestCase, answer: &str) -> Result<(), Box<dyn fmt::Display>>;
}

pub trait Checker: Sync + Check {}
impl<T> Checker for T where T: Sync + Check {}
