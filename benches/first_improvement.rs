use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use onemax::algorithm::Algorithm;
use onemax::heuristics::FirstImprovement;
use onemax::mutation_operator::Bitflip;
use onemax::solution::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Square Numbers");

    for &num in &[5, 10, 20] {
        group.bench_with_input(BenchmarkId::from_parameter(num), &num, |b, &num| {
            b.iter(|| {
                let mut solution = Solution::new(num);

                let bitflip = Bitflip { p: 1.0 };

                let first_improvement = FirstImprovement {
                    rounds: 5,
                    operator: Box::new(bitflip),
                };

                // let start = Instant::now();
                black_box(first_improvement.apply(&mut solution));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
