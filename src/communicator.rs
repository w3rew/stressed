use crate::utils::{ensure_newline, trim_lines, ProgramFailure};
use std::error::Error;
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
        self.communicate_result(input, args).await.unwrap()
    }

    pub async fn communicate_result(
        &self,
        input: Option<&str>,
        args: Option<&[&str]>,
    ) -> Result<String, Box<dyn Error>> {
        let mut command = Command::new(&self.executable);

        match input {
            Some(_) => command.stdin(Stdio::piped()),
            None => command.stdin(Stdio::null()),
        };
        if let Some(args) = args {
            command.args(args);
        };
        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut prog = command.spawn()?;
        drop(command);

        if let Some(input) = input {
            let mut stdin = prog.stdin.take().unwrap();
            stdin.write(input.as_bytes()).await?;
            drop(stdin);
        }

        let result = prog.wait_with_output().await?;

        let success = result.status.success();
        let mut answer = String::from_utf8(result.stdout)?;

        if self.trim_output {
            answer = trim_lines(&answer);
        }
        ensure_newline(&mut answer);
        match success {
            true => Ok(answer),
            false => {
                let err_out = String::from_utf8(result.stderr)?;
                Err(Box::new(ProgramFailure::new(err_out)))
            }
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
