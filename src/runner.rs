use crate::utils::{SeedType, TestCase};
use crate::checker::Check;
use crate::sampler::Sampler;
use crate::solver::Solver;
use std::fmt;
pub fn run_sequence(generator: &Sampler,
                    prog: &Solver,
                    checker: &dyn Check,
                    seed_start: SeedType,
                    seed_step: usize) -> Result<(), Box<dyn fmt::Display>> {
    for seed in (seed_start..=SeedType::MAX).step_by(seed_step) {
        let sample = generator.sample(seed);
        let testcase = TestCase::new(seed, sample);
        let answer = prog.interact(&testcase.body);
        checker.check(&testcase, &answer)?;
    }
    Ok(())
}
