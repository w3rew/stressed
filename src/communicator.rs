use crate::utils::{ensure_newline, trim_lines};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct Communicator {
    executable: PathBuf,
    trim_output: bool,
}

impl Communicator {
    pub fn new(executable: PathBuf, trim_output: bool) -> Communicator {
        Communicator {
            executable,
            trim_output,
        }
    }

    pub async fn communicate(&self, input: Option<&str>, args: Option<&[&str]>) -> String {
        match self.communicate_result(input, args).await {
            Ok(x) => x,
            Err(x) => x,
        }
    }

    pub async fn communicate_result(
        &self,
        input: Option<&str>,
        args: Option<&[&str]>,
    ) -> Result<String, String> {
        let mut command = Command::new(&self.executable);

        match input {
            Some(_) => command.stdin(Stdio::piped()),
            None => command.stdin(Stdio::null()),
        };
        if let Some(args) = args {
            command.args(args);
        };
        command.stdout(Stdio::piped()).stderr(Stdio::null());

        let mut prog = command.spawn().expect(&format!(
            "Couldn't spawn child process {}",
            self.executable.to_str().unwrap()
        ));
        drop(command);

        if let Some(input) = input {
            let mut stdin = prog.stdin.take().unwrap();
            stdin.write(input.as_bytes()).await.expect(
                "The program refused to read the data. \
                Check your program's input",
            );
            drop(stdin);
        }

        let result = prog
            .wait_with_output()
            .await
            .expect("Could not communicate");
        let success = result.status.success();
        let mut answer = String::from_utf8(result.stdout).expect("Could not decode output");

        if self.trim_output {
            answer = trim_lines(&answer);
        }
        ensure_newline(&mut answer);

        match success {
            true => Ok(answer),
            false => Err(answer),
        }
    }
}

impl<T> From<T> for Communicator
where
    PathBuf: From<T>,
{
    fn from(value: T) -> Communicator {
        Communicator::new(PathBuf::from(value), false)
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_communicator() {
        let c = Communicator::from("cat");

        for i in 0..100 {
            let i_string = format!("{i}\n");
            let ans = c.communicate(Some(&i_string), None).await;
            assert_eq!(ans, i_string);
        }
    }
}
