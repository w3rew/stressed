use crate::utils::SeedType;

#[derive(Clone)]
pub struct TestCase {
    pub seed: SeedType,
    pub body: String
}

impl TestCase {
    pub fn new(seed: SeedType, body: String) -> TestCase {
        TestCase{seed, body}
    }
}
