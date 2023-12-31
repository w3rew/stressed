use crate::communicator::Communicator;
use std::error::Error;
use std::path::PathBuf;

pub struct Solver {
    c: Communicator,
}

impl Solver {
    pub fn new(executable: PathBuf, trim_output: bool) -> Solver {
        Solver {
            c: Communicator::new(executable, trim_output),
        }
    }

    pub async fn solve(&self, input: &str) -> Result<String, Box<dyn Error>> {
        self.c.communicate_result(Some(input), None).await
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_solver() {
        let solver = Solver::new(PathBuf::from("cat"), false);
        for i in 0..100 {
            let i_string = format!("{i}\n");
            let ans = solver.solve(&i_string).await.unwrap();
            assert_eq!(ans, i_string);
        }
    }
}
