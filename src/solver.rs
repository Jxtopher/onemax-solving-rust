use crate::algorithm::Algorithm;
use crate::solution::Solution;

pub struct Solver {
    pub metaheuristic: Box<dyn Algorithm>,
}

impl Algorithm for Solver {
    fn apply(&self, solution: &mut Solution) {
        self.metaheuristic.apply(solution);
    }
}
