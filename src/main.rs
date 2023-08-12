mod args;
mod utils;
mod checker;
mod communicator;
mod runner;
mod sampler;
mod solver;

use crate::args::parse_args;
use crate::checker::{Checker, DefaultChecker, CustomChecker};
use crate::runner::run_sequence;
use crate::sampler::Sampler;
use crate::solver::Solver;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let args = parse_args();

    let sampler = Sampler::new(args.sampler_path, args.sampler_use_stdin);

    let prog = Solver::new(args.solver_path);
    let checker: Box<dyn Checker> = if args.custom_checker {
        Box::new(CustomChecker::from(args.checker_path))
    } else {
        Box::new(DefaultChecker::new(args.checker_path, args.diff_mode))
    };


    let result = run_sequence(&sampler,
                              &prog,
                              &*checker,
                              args.niter,
                              args.progress).await;

    if let Err(display) = result {
        eprint!("{display}");
        Err(std::io::Error::from_raw_os_error(1))
    } else {
        println!("Tests passed!");
        std::io::Result::Ok(())
    }

}
