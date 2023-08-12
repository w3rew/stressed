use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "stressed")]
pub struct Args {
    /// Path to solver
    pub solver_path: PathBuf,

    /// Path to sampler
    #[arg(short = 's', long = "sampler", value_name = "sampler_path")]
    pub sampler_path: PathBuf,

    /// Path to checker: either to reference solver, or to the dedicated
    /// checker. See custom_checker for details.
    #[arg(short = 'c', long = "check", value_name = "check")]
    pub checker_path: PathBuf,

    /// Whether to use custom checker. Without this flag checker argument is
    /// interpreted as path to the reference solver and the output of solution
    /// is compared to the reference solver's. However, if custom_checker flag
    /// is present, checker receives *testcase* and, **immediately after**, *the program's answer*
    #[arg(long = "custom_checker")]
    pub custom_checker: bool,

    /// Use stdin to supply random seed to sampler. The default behaviour is
    /// to specify it as the only argument to the sampler.
    #[arg(long = "sampler_use_stdin")]
    pub sampler_use_stdin : bool,

    /// Mode to use for diffs; works only for default checker
    #[arg(long = "line_diff")]
    pub diff_mode : DiffMode,

    #[arg(short, long)]
    pub debug: bool,

    /// Show progress bar
    #[arg(long)]
    pub progress: bool,

    /// Number of samples to try
    #[arg(short = 'n', long = "niter", default_value_t = 1000000)]
    pub niter: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DiffMode {
    /// Output diff per line
    Line,

    /// Output diff per character
    Char,

    /// Do not output diff at all; instead, just output what the tested program answered.
    /// This might be desirable since the reference solver's output is printed anyway.
    None
}

pub fn parse_args() -> Args {
    Args::parse()
}
