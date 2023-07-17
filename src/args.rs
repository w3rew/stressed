use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "stresser")]
pub struct Args {
    pub solver_path: PathBuf,

    #[arg(short = 's', long = "sampler", value_name = "sampler_path")]
    pub sampler_path: PathBuf,

    #[command(flatten)]
    pub checker: Checkers,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long)]
    pub progress: bool,
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
