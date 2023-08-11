use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "stressed")]
pub struct Args {
    pub solver_path: PathBuf,

    #[arg(short = 's', long = "sampler", value_name = "sampler_path")]
    pub sampler_path: PathBuf,

    #[arg(long = "sampler_input", default_value_t = SamplerInput::Arg, value_enum)]
    pub sampler_input : SamplerInput,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long)]
    pub progress: bool,

    #[arg(short = 'n', long = "niter", default_value_t = 1000000)]
    pub niter: usize,

    #[command(flatten)]
    pub checker: Checkers,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum SamplerInput {
    Stdin,
    Arg,
}


#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Checkers {
    #[arg(short = 'r', long = "reference", 
          value_name = "reference_path", required = false)]
    pub default: Option<PathBuf>,

    #[arg(short = 'c', long = "checker",
          value_name = "checker_path", required = false)]
    pub custom: Option<PathBuf>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
