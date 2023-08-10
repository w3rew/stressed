use std::path::PathBuf;
use tokio::process::Command;
use std::process::Stdio;
use crate::utils::ensure_newline;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Solver {
    executable: PathBuf,
}

impl Solver {
    pub fn new(executable: PathBuf) -> Solver {
        Solver{executable}
    }

    pub async fn interact(&self, s: &str) -> String {
        let mut command = Command::new(&self.executable);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut prog = match command.spawn() {
                Err(why) => panic!("Couldn't spawn solution process: {why}"),
                Ok(prog) => prog,
            };
        drop(command);

        let mut stdin = prog.stdin.take().unwrap();
        stdin.write(s.as_bytes()).await.unwrap();
        drop(stdin);

        let output = prog.wait_with_output().await.expect("Could not solve using solver").stdout;
        let mut answer = String::from_utf8(output).expect("Could not decode sampler output");

        ensure_newline(&mut answer);
        answer
    }
}

impl<T> From<T> for Solver where PathBuf: From<T> {
    fn from(value: T) -> Solver {
        Solver::new(PathBuf::from(value))
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    use std::fmt;
    #[test]
    fn echo_solver() {
        let solver = Solver::from("cat");

        for i in 0..100 {
            let mut i_string = format!("{i}\n");
            let ans = solver.interact(&i_string);
            println!("ans={ans} i_string={i_string}");
            assert_eq!(ans, i_string);
        }
    }
}
