use log::info;

use crate::solution::Solution;

pub fn first_improvement(solution: &mut Solution, operator: fn(&mut Solution, f64), rounds: u64) {
    let mut round = 0;
    while round < rounds && solution.get_fitness() < solution.get_size().try_into().unwrap() {
        operator(solution, 0.01);

        if solution.get_change_fitness() > solution.get_fitness() {
            solution.commit();
        } else {
            solution.revert_changes();
        }

        info!("{:03} {}", round, solution);
        round += 1;
    }
}
