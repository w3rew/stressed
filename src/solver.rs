use crate::communicator::Communicator;
use std::path::PathBuf;

pub struct Solver {
    c: Communicator,
}

impl Solver {
    pub fn new(executable: PathBuf) -> Solver {
        Solver {
            c: Communicator::new(executable),
        }
    }

    pub async fn solve(&self, input: &str) -> String {
        self.c.communicate(Some(input), None).await
    }
}

#[cfg(all(target_os = "linux", test))]
mod tests {
    use super::*;
    #[tokio::test]
    async fn echo_solver() {
        let solver = Solver::new(PathBuf::from("cat"));
        for i in 0..100 {
            let i_string = format!("{i}\n");
            let ans = solver.solve(&i_string).await;
            assert_eq!(ans, i_string);
        }
    }
}
