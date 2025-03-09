use crate::solution::Solution;

pub trait Algorithm {
    fn apply(&self, solution: &mut Solution);
    fn name(&self) -> &'static str;
}
