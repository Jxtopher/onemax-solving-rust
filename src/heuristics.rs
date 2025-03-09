use log::info;

use crate::algorithm::Algorithm;
use crate::solution::Solution;

pub struct FirstImprovement {
    pub rounds: u64,
    pub operator: Box<dyn Algorithm>,
}

impl Algorithm for FirstImprovement {
    fn apply(&self, solution: &mut Solution) {
        let mut round = 0;
        while round < self.rounds
            && solution.get_fitness() < solution.get_size().try_into().unwrap()
        {
            self.operator.apply(solution);

            if solution.get_change_fitness() > solution.get_fitness() {
                solution.commit();
            } else {
                solution.revert_changes();
            }

            info!("{:03} {}", round, solution);
            round += 1;
        }
    }
    fn name(&self) -> &'static str {
        "FirstImprovement"
    }
}
