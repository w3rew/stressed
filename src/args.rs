use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "stressed")]
pub struct Args {
    pub solver_path: PathBuf,

    #[arg(short = 's', long = "sampler", value_name = "sampler_path")]
    pub sampler_path: PathBuf,

    #[arg(short = 'c', long = "check", value_name = "check")]
    pub checker_path: PathBuf,

    #[arg(long = "custom_checker")]
    pub custom_checker: bool,

    #[arg(long = "sampler_use_stdin")]
    pub sampler_use_stdin : bool,

    #[arg(long = "line_diff")]
    pub line_diff : bool,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(long)]
    pub progress: bool,

    #[arg(short = 'n', long = "niter", default_value_t = 1000000)]
    pub niter: usize,
}

pub fn parse_args() -> Args {
    Args::parse()
}
