use crate::utils::{SeedType, TestCase};
use crate::checker::Checker;
use crate::sampler::Sampler;
use crate::solver::Solver;
use indicatif::ProgressBar;
use std::fmt;
use futures::prelude::*;
use futures::stream::FuturesUnordered;

const WORKERS_BUF_SIZE: usize = 32;
const BAR_STEP: usize = 10;

pub async fn run_sequence(generator: &Sampler,
                    prog: &Solver,
                    checker: &dyn Checker,
                    niter: usize,
                    progress: bool) -> Result<(), Box<dyn fmt::Display>> {
    let bar = match progress {
        true => ProgressBar::new(niter.try_into().unwrap()),
        false => ProgressBar::hidden()
    };

    let (task_tx, task_rx) = async_channel::bounded(WORKERS_BUF_SIZE);

    let mut futs = FuturesUnordered::new();

    tokio::spawn(async move {
        let mut seed = SeedType::MIN;
        for _ in 0..niter {
            task_tx.send(seed).await;
            seed += 1;
        }
    });

    for _ in 0..niter {
        futs.push(async {
            let seed = task_rx.recv().await.unwrap();
            let sample = generator.sample(seed).await;
            let testcase = TestCase::new(seed, sample);
            let answer = prog.interact(&testcase.body).await;
            let result = checker.check(&testcase, &answer).await;
            if let Err(e) = result {
                println!("err");
                Err(e)
            }
            else {
                Ok(())
            }
        });
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
