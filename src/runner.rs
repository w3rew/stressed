use crate::utils::{SeedType, TestCase};
use crate::Checker;
use crate::{Sampler, Solver};
use indicatif::ProgressBar;
use std::fmt;
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use tokio::sync::Semaphore;

const WORKERS_PERMITS: usize = 32;
const BAR_STEP: usize = 20;

pub async fn run_sequence(generator: &Sampler,
                          prog: &Solver,
                          checker: &dyn Checker,
                          niter: usize,
                          progress: bool) -> Result<(), Box<dyn fmt::Display>> {
    let bar = match progress {
        true => ProgressBar::new(niter.try_into().unwrap()),
        false => ProgressBar::hidden()
    };

    let mut seed = SeedType::MIN;

    let mut futs = FuturesUnordered::new();

    // Here we have a semaphore, which does not allow more than
    // WORKERS_PERMITS threads to be in section, where file desctiptors
    // are allocated. This helps the program to progress and makes sure that
    // file descriptor limit is not hit.
    //
    // The common decision to make sure the semaphore is accessible to all fibers is
    // Arc. However, in this case we know that the main fiber awaits all its child
    // fibers, so the lifetime is correct. That's why the hackery with transmute is safe.
    let fds_semaphore = Semaphore::new(WORKERS_PERMITS);
    let fds_semaphore_ptr = &fds_semaphore as *const Semaphore;
    let fds_semaphore_ref: &'static Semaphore = unsafe {
        std::mem::transmute::<*const Semaphore, &'static Semaphore>(fds_semaphore_ptr)
    };

    for _ in 0..niter {
        let cur_seed = seed.clone();
        let generator = &generator;
        let prog = &prog;
        let checker = &checker;
        futs.push(async move {
            let permit = fds_semaphore_ref.acquire().await.unwrap();
            let sample = generator.sample(cur_seed).await;
            let testcase = TestCase::new(cur_seed, sample);
            let answer = prog.solve(&testcase.body).await;
            let result = checker.check(&testcase, &answer).await;
            drop(permit);

            if let Err(e) = result {
                Err(e)
            }
            else {
                Ok(())
            }
        });
        seed += 1;
    }

    let mut completed: usize = 0;

    loop {
        match futs.next().await {
            None => {
                bar.finish();
                return Ok(());
            },
            Some(Err(e)) => {
                bar.finish();
                return Err(e);
            },
            Some(Ok(_)) => {
                completed += 1;
                if completed % BAR_STEP == 0 {
                    bar.inc(BAR_STEP as u64);
                }
            },
        }
    }
    unreachable!()
}
