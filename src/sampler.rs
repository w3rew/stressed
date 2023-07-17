use std::path::PathBuf;
use std::process::Command;
use crate::utils::{SeedType, ensure_newline};


pub struct Sampler {
    executable: PathBuf,
}

impl Sampler {
    pub fn new(executable: &str) -> Sampler {
        Sampler{executable: PathBuf::from(executable)}
    }

    pub fn sample(&self, seed: SeedType) -> String {
        let bytes = Command::new(&self.executable)
                              .arg(seed.to_string())
                              .output()
                              .expect("Could not run sampler")
                              .stdout;

        let mut ans = String::from_utf8(bytes).expect("Could not decode sampler output");
        ensure_newline(&mut ans);
        ans
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[test]
    fn echo_sampler() {
        let echo_sampler = Sampler::new("echo");
        for i in 0..100 {
            let ans = echo_sampler.sample(i as SeedType);
            println!("{i} {ans}");
            assert_eq!(ans.trim().parse::<SeedType>().unwrap(), i);
        }
    }
}
