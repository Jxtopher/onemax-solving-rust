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
}

#[cfg(test)]
mod tests {
    use crate::mutation_operator::Bitflip;

    use super::*;

    #[test]
    fn first_improvement() {
        let mut solution = Solution::new(5);
        let bitflip = Bitflip { p: 1.0 };

        let first_improvement = FirstImprovement {
            rounds: 5,
            operator: Box::new(bitflip),
        };

        first_improvement.apply(&mut solution);

        assert_eq!(5, solution.get_fitness());
    }
}
