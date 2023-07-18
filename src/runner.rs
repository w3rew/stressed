use crate::utils::{SeedType, TestCase};
use crate::checker::Check;
use crate::sampler::Sampler;
use crate::solver::Solver;
use indicatif::ProgressBar;
use std::fmt;
pub fn run_sequence(generator: &Sampler,
                    prog: &Solver,
                    checker: &dyn Check,
                    seed_start: SeedType,
                    niter: usize,
                    progress: bool) -> Result<(), Box<dyn fmt::Display>> {
    let bar = match progress {
        true => ProgressBar::new(niter.try_into().unwrap()),
        false => ProgressBar::hidden()
    };
    for seed in seed_start..(seed_start + TryInto::<SeedType>::try_into(niter).unwrap()) {
        if seed % 20 == 0 {
            bar.set_position((seed - seed_start) as u64);
        }
        let sample = generator.sample(seed);
        let testcase = TestCase::new(seed, sample);
        let answer = prog.interact(&testcase.body);
        checker.check(&testcase, &answer)?;
    }
    bar.finish();
    Ok(())
}
