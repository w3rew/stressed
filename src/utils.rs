mod messages;
mod compare_error;

pub type SeedType = i64;

pub use messages::TestCase;
pub use compare_error::CompareError;
use std::process::{Termination, ExitCode};

pub fn ensure_newline(s: &mut String) {
    s.truncate(s.trim_end().len());
    s.push_str("\n");
}

pub enum SilentResult {
    Ok,
    Error
}

impl Termination for SilentResult {
    fn report(self) -> ExitCode {
        match self {
            SilentResult::Ok => ExitCode::SUCCESS,
            SilentResult::Error => ExitCode::FAILURE,
        }
    }
}
