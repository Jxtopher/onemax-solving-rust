// $env:RUST_LOG="DEBUG"
// $env:RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
mod args;
mod globals;
mod heuristics;
mod mutation_operator;
mod solution;

use args::Args;
use globals::initialize_globals;
use heuristics::first_improvement;
use mutation_operator::bitflip;

use clap::Parser;
use env_logger;
use log::info;
use solution::Solution;

fn main() {
    env_logger::init();
    info!("Solve OneMax problem");
    let args = Args::parse();
    initialize_globals(args.seed);

    let mut solution = Solution::new(args.n);

    first_improvement(&mut solution, bitflip, args.rounds);
}
