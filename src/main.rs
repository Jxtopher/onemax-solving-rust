// $env:RUST_LOG="DEBUG"
// $env:RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build

use clap::Parser;
use log::info;

use onemax::algorithm::Algorithm;
use onemax::args::Args;
use onemax::factory::factory;
use onemax::globals::initialize_globals;
use onemax::heuristics::FirstImprovement;
use onemax::mutation_operator::Bitflip;
use onemax::mutation_operator::Kbit;
use onemax::solution::Solution;

fn main() {
    env_logger::init();
    info!("Solve OneMax problem");
    let args = Args::parse();
    initialize_globals(args.seed);

    let mut solution = Solution::new(args.n);

    let solver = factory(args.configuration);
    solver.apply(&mut solution);

    let bitflip = Bitflip { p: 0.01 };

    let first_improvement = FirstImprovement {
        rounds: args.rounds,
        operator: Box::new(bitflip),
    };

    first_improvement.apply(&mut solution);

    solution.reset();

    let kbit = Kbit { k: 5 };
    let first_improvement_2 = FirstImprovement {
        rounds: args.rounds,
        operator: Box::new(kbit),
    };
    first_improvement_2.apply(&mut solution);
}
