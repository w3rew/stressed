mod args;
mod utils;
mod sampler;
mod solver;
mod checker;

use crate::args::parse_args;
use crate::sampler::Sampler;
use crate::solver::Solver;
use crate::checker::{Check, DefaultChecker};
use crate::utils::TestCase;

fn main() {
    let args = parse_args();

    let mut sample = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut sample) {
        panic!("Something went wrong.");
    }

    let sample = TestCase::new(123, sample);

    let prog = Solver::new("./test_prog");
    let checker: Box<dyn Check> = Box::new(DefaultChecker::new("./reference_prog"));

    let out = prog.interact(&sample.body);

    match checker.check(&sample, &out) {
        Ok(_) => println!("Ok!"),
        Err(e) => print!("{e}")
    }

}
