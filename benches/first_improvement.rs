use criterion::{black_box, criterion_group, criterion_main, Criterion};

use onemax::algorithm::Algorithm;
use onemax::heuristics::FirstImprovement;
use onemax::mutation_operator::Bitflip;
use onemax::solution::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut solution = Solution::new(5);

    let bitflip = Bitflip { p: 1.0 };

    let first_improvement = FirstImprovement {
        rounds: 5,
        operator: Box::new(bitflip),
    };

    c.bench_function("Frist inprovement x", |b| {
        b.iter(|| first_improvement.apply(&mut solution))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
