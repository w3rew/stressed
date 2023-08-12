use crate::communicator::Communicator;
use crate::utils::SeedType;
use std::path::PathBuf;

pub struct Sampler {
    c: Communicator,
    use_stdin: bool,
}

impl Sampler {
    pub fn new(executable: PathBuf, use_stdin: bool) -> Sampler {
        Sampler {
            c: Communicator::new(executable),
            use_stdin,
        }
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

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn seed_as_arg() {
        let arg_sampler = Sampler::new(PathBuf::from("echo"), false);
        for seed in 0..100 {
            let ans = arg_sampler.sample(seed as SeedType).await;
            let correct_ans = format!("{seed}\n");
            let incorrect_ans = format!("{}\n", seed + 1);
            assert_eq!(ans, correct_ans);
            assert_ne!(ans, incorrect_ans);
        }
    }
    #[tokio::test]
    async fn seed_as_stdin() {
        let arg_sampler = Sampler::new(PathBuf::from("cat"), true);
        for seed in 0..100 {
            let ans = arg_sampler.sample(seed as SeedType).await;
            let correct_ans = format!("{seed}\n");
            assert_eq!(ans, correct_ans);
        }
    }
}
