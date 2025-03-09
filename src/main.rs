// $env:RUST_LOG="DEBUG"
// $env:RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
mod algorithm;
mod args;
mod globals;
mod heuristics;
mod mutation_operator;
mod solution;

use algorithm::Algorithm;
use args::Args;
use globals::initialize_globals;
use heuristics::FirstImprovement;
use mutation_operator::Bitflip;
use mutation_operator::Kbit;

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

    let bitflip = Bitflip { p: 0.01 };

    let first_improvement = FirstImprovement {
        rounds: args.rounds,
        operator: Box::new(bitflip),
    };

    first_improvement.apply(&mut solution);

    solution.reset();

    println!("xxxxxxxxxxx");
    let kbit = Kbit { k: 5 };
    let first_improvement_2 = FirstImprovement {
        rounds: args.rounds,
        operator: Box::new(kbit),
    };
    first_improvement_2.apply(&mut solution);
}
