use std::path::PathBuf;
use tokio::process::Command;
use crate::utils::{SeedType, ensure_newline};
use std::process::Stdio;
use std::os::fd::AsFd;


pub struct Sampler {
    executable: PathBuf,
}

impl Sampler {
    pub fn new(executable: PathBuf) -> Sampler {
        Sampler{executable}
    }

    pub async fn sample(&self, seed: SeedType) -> String {
        let mut command = Command::new(&self.executable);
        command.stdin(Stdio::null()).stderr(Stdio::null()).stdout(Stdio::piped());
        let prog = match command.spawn() {
            Err(why) => panic!("Couldn't spawn sampler process: {why}"),
            Ok(prog) => prog,
        };
        drop(command);
        let output = prog.wait_with_output().await.expect("Could not sample");

        let mut testcase = String::from_utf8(output.stdout).expect("Could not decode sampler output");
        ensure_newline(&mut testcase);
        testcase
    }
}

impl<T> From<T> for Sampler where PathBuf: From<T> {
    fn from (value: T) -> Sampler {
        Sampler::new(PathBuf::from(value))
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[test]
    fn echo_sampler() {
        let echo_sampler = Sampler::from("echo");
        for i in 0..100 {
            let ans = echo_sampler.sample(i as SeedType);
            println!("{i} {ans}");
            assert_eq!(ans.trim().parse::<SeedType>().unwrap(), i);
        }
    }
}
