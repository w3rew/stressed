use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "stressed")]
pub struct Args {
    /// Path to solver
    pub solver_path: PathBuf,

    /// Path to sampler
    #[arg(
        short = 's',
        long = "sampler",
        value_name = "sampler_path",
        visible_alias = "generator"
    )]
    pub sampler_path: PathBuf,

    /// Path to checker: either to reference solver, or to the dedicated
    /// checker. See --use-custom-checker for details.
    #[arg(short = 'c', long = "checker", value_name = "check")]
    pub checker_path: PathBuf,

    /// Whether to use custom checker. Without this flag, --checker argument is
    /// interpreted as path to the reference solver. If --use-custom-checker flag
    /// is present, --checker receives *testcase* and, **immediately after**, *the program's answer*
    /// and should exit with zero or non-zero exit code.
    #[arg(long = "use-custom-checker")]
    pub custom_checker: bool,

    /// Use stdin to supply random seed to sampler. The default behaviour is
    /// to specify it as the only argument to the sampler.
    #[arg(long = "sampler-use-stdin")]
    pub sampler_use_stdin: bool,


    /// Do not trim output of solvers. The default
    /// behaviour is to trim every line.
    /// Specify this option to disable trimming.
    #[arg(long = "no-trim-output")]
    pub no_trim_output: bool,

    /// Mode to use for diffs; works only for default checker
    #[arg(value_enum, long = "diff-mode", default_value_t = DiffMode::Line)]
    pub diff_mode: DiffMode,

    // #[arg(short, long)]
    // pub debug: bool,
    /// Do not show progress bar
    #[arg(long = "no-progress")]
    pub no_progress: bool,

    /// Number of samples to try
    #[arg(short = 'n', long = "niter", default_value_t = 10000)]
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
    None,
}

pub fn parse_args() -> Args {
    Args::parse()
}
