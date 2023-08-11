mod args;
mod utils;
mod checker;
mod communicator;
mod runner;

use crate::args::parse_args;
use crate::communicator::Communicator;
use crate::checker::{Checker, DefaultChecker, CustomChecker};
use crate::runner::run_sequence;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = parse_args();

    let sampler = Communicator::new(args.sampler_path);
    let prog = Communicator::new(args.solver_path);

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
