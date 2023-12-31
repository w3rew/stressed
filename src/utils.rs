mod messages;

pub type SeedType = u64;

pub use messages::{ProgramFailure, TestCase, TestResult, DELIMITER_STR};
use std::process::{ExitCode, Termination};

pub fn trim_lines(s: &str) -> String {
    let lines = s.lines();

    lines.map(|x| x.trim()).collect::<Vec<_>>().join("\n")
}

pub fn ensure_newline(s: &mut String) {
    s.truncate(s.trim_end().len());
    s.push_str("\n");
}

pub enum SilentResult {
    Ok,
    Error,
}

impl Termination for SilentResult {
    fn report(self) -> ExitCode {
        match self {
            SilentResult::Ok => ExitCode::SUCCESS,
            SilentResult::Error => ExitCode::FAILURE,
        }
    }
}
