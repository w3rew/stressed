use crate::checker::CheckerError;
use crate::utils::{SeedType, TestCase, TestResult};
use crate::{Checker, Sampler, Solver};
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

const WORKERS_PERMITS: usize = 16;
const BAR_STEP: usize = 50;
const PROGRESS_BAR_TEMPLATE: &str = "{wide_bar} | {pos}/{len} \
                                     [{elapsed_precise}<{eta_precise}]";

macro_rules! return_if_cancelled {
    ($default:expr, $alternative:expr, $if_cancelled:expr) => {
        tokio::select! {
            _my_return = $default => {
                _my_return
            },
            _ = $alternative => {
                return $if_cancelled;
            }
        }
    };
}

async fn run_one<'a>(
    generator: &'a Sampler,
    prog: &'a Solver,
    checker: &'a dyn Checker,
    seed: SeedType,
) -> TestResult {
    let sample = match generator.sample(seed).await {
        Err(e) => return TestResult::SamplerError(e),
        Ok(s) => s,
    };
    let testcase = TestCase::new(seed, sample);
    let answer = match prog.solve(&testcase.body).await {
        Ok(s) => s,
        Err(err) => {
            return TestResult::SolutionRuntimeError { testcase, err };
        }
    };
    match checker.check(&testcase, &answer).await {
        Ok(()) => TestResult::Ok,
        Err(CheckerError::RuntimeError(err)) => TestResult::CheckerError { testcase, err },
        Err(CheckerError::WrongAnswer(msg)) => TestResult::WrongAnswer { testcase, msg },
    }
}

pub async fn run_sequence(
    generator: &Sampler,
    prog: &Solver,
    checker: &dyn Checker,
    niter: usize,
    progress: bool,
) -> TestResult {
    let bar = match progress {
        true => ProgressBar::new(niter.try_into().unwrap())
            .with_style(ProgressStyle::with_template(PROGRESS_BAR_TEMPLATE).unwrap()),
        false => ProgressBar::hidden(),
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
    let fds_semaphore_ref: &'static Semaphore =
        unsafe { std::mem::transmute::<*const Semaphore, &'static Semaphore>(fds_semaphore_ptr) };

    let cancel_token = CancellationToken::new();

    for _ in 0..niter {
        let cur_seed = seed.clone();
        let generator = &generator;
        let prog = &prog;
        let checker = &checker;
        let cur_cancel_token = cancel_token.clone();
        futs.push(async move {
            // We are concerned with cancellation only here as waiting in line
            // for semaphore token is the most expensive part.
            // Sampling, solving and checking are probably not worth the
            // trouble as both generator and program are supposed to be very
            // fast, although it's worth investigating
            let _permit = return_if_cancelled!(
                fds_semaphore_ref.acquire(),
                cur_cancel_token.cancelled(),
                TestResult::Ok
            );

            run_one(generator, prog, *checker, cur_seed).await
        });
        seed += 1;
    }

    let mut completed: usize = 0;

    let mut result = TestResult::Ok;

    loop {
        match futs.next().await {
            None => {
                break;
            }
            Some(TestResult::Ok) => {
                completed += 1;
                if completed % BAR_STEP == 0 {
                    bar.inc(BAR_STEP as u64);
                }
            }
            Some(x) => {
                if let TestResult::Ok = result {
                    cancel_token.cancel();
                    result = x;
                }
                // Early printing hack: if we print the result only in main,
                // we have to wait for all threads to finish.
                // Maybe change if cancellation works fine.
            }
        }
    }
    bar.finish();
    result
}
