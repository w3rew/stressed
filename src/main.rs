use stressed::args::parse_args;
use stressed::checker::{Checker, DefaultChecker, CustomChecker};
use stressed::runner::run_sequence;
use stressed::sampler::Sampler;
use stressed::solver::Solver;
use stressed::utils::SilentResult;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> SilentResult {
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
        SilentResult::Error
    } else {
        println!("Tests passed!");
        SilentResult::Ok
    }

}
