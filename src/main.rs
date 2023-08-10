mod args;
mod utils;
mod sampler;
mod solver;
mod checker;
mod runner;

use crate::args::parse_args;
use crate::sampler::Sampler;
use crate::solver::Solver;
use crate::checker::{Checker, DefaultChecker, CustomChecker};
use crate::runner::run_sequence;
use std::process;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // println!("{}", process::id());
    let args = parse_args();

    let sampler = Sampler::new(args.sampler_path);
    let prog = Solver::new(args.solver_path);

    let checker: Box<dyn Checker> = match (args.checker.default, args.checker.custom) {
        (Some(ref reference_path), None) => Box::new(DefaultChecker::from(reference_path)),
        (None, Some(ref checker_path)) => Box::new(CustomChecker::from(checker_path)),
        _ => unreachable!("Wrong checker/reference argument combination")
    };


    let result = run_sequence(&sampler,
                              &prog,
                              &*checker,
                              args.niter,
                              args.progress).await;

    if let Err(display) = result {
        eprint!("{display}");
    } else {
        println!("Tests passed!");
    }

}
