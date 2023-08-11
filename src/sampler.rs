use std::path::PathBuf;
use crate::communicator::Communicator;
use crate::utils::SeedType;

pub struct Sampler {
    c: Communicator,
    use_stdin: bool
}

impl Sampler {
    pub fn new(executable: PathBuf, use_stdin: bool) -> Sampler {
        Sampler{c: Communicator::new(executable), use_stdin}
    }

    pub async fn sample(&self, seed: SeedType) -> String {
        let seed_string = seed.to_string();
        if self.use_stdin {
            self.c.communicate(Some(&seed_string), None).await
        } else {
            self.c.communicate(None, Some(&[&seed_string])).await
        }
    }
}
