use std::path::PathBuf;
use crate::communicator::Communicator;
use crate::utils::SeedType;

pub struct Solver {
    c: Communicator,
}

impl Solver {
    pub fn new(executable: PathBuf) -> Solver {
        Solver{c: Communicator::new(executable)}
    }

    pub async fn solve(&self, input: &str) -> String {
        self.c.communicate(Some(input), None).await
    }
}
