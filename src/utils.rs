mod messages;
mod compare_error;

pub type SeedType = i64;

pub use messages::TestCase;
pub use compare_error::CompareError;

pub fn ensure_newline(s: &mut String) {
    s.truncate(s.trim_end().len());
    s.push_str("\n");
}
