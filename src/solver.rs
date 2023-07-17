use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{Read, Write};
use crate::utils::ensure_newline;

pub struct Solver {
    executable: PathBuf,
}

impl Solver {
    pub fn new(executable: PathBuf) -> Solver {
        Solver{executable}
    }

    pub fn interact(&self, s: &str) -> String {
        let prog = match Command::new(&self.executable)
                                  .stdin(Stdio::piped())
                                  .stdout(Stdio::piped())
                                  .spawn() {
                                      Err(why) => panic!("Couldn't spawn solution process: {why}"),
                                      Ok(prog) => prog,
                                  };

        if let Err(why) = prog.stdin.unwrap().write_all(s.as_bytes()) {
            panic!("Couldn't write sample to solution: {why}");
        }

        let mut output = String::new();

        if let Err(why) = prog.stdout.unwrap().read_to_string(&mut output) {
            panic!("Could not read response from solution: {why}");
        }

        ensure_newline(&mut output);
        output
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
