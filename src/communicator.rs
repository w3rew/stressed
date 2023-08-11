use std::path::PathBuf;
use tokio::process::Command;
use std::process::Stdio;
use crate::utils::ensure_newline;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Communicator {
    executable: PathBuf,
}

impl Communicator {
    pub fn new(executable: PathBuf) -> Communicator {
        Communicator{executable}
    }

    pub async fn communicate(&self, input: Option<&str>, args: Option<&[&str]>) -> String {
        let mut command = Command::new(&self.executable);

        match input {
            Some(_) => command.stdin(Stdio::piped()),
            None => command.stdin(Stdio::null())
        };
        if let Some(args) = args {
            command.args(args);
        };
        command.stdout(Stdio::piped()).stderr(Stdio::null());

        let mut prog = match command.spawn() {
                Err(why) => panic!("Couldn't spawn child process: {why}"),
                Ok(prog) => prog,
            };
        drop(command);

        if let Some(input) = input {
            let mut stdin = prog.stdin.take().unwrap();
            stdin.write(input.as_bytes()).await.unwrap();
            drop(stdin);
        }

        let output = prog.wait_with_output().await.expect("Could not communicate").stdout;
        let mut answer = String::from_utf8(output).expect("Could not decode output");

        ensure_newline(&mut answer);
        answer
    }
}

impl<T> From<T> for Communicator where PathBuf: From<T> {
    fn from(value: T) -> Communicator {
        Communicator::new(PathBuf::from(value))
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    use std::fmt;
    #[test]
    fn echo_solver() {
        let solver = Communicator::from("cat");

        for i in 0..100 {
            let mut i_string = format!("{i}\n");
            let ans = solver.communicate(Some(&i_string), None);
            println!("ans={ans} i_string={i_string}");
            assert_eq!(ans, i_string);
        }
    }
}
