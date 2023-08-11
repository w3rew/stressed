mod args;
mod utils;
mod checker;
mod communicator;
mod runner;
mod sampler;
mod solver;

use crate::args::{parse_args, SamplerInput};
use crate::checker::{Checker, DefaultChecker, CustomChecker};
use crate::runner::run_sequence;
use crate::sampler::Sampler;
use crate::solver::Solver;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = parse_args();

    let sampler = match args.sampler_input {
        SamplerInput::Arg => Sampler::new(args.sampler_path, false),
        SamplerInput::Stdin => Sampler::new(args.sampler_path, true),
    };
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
