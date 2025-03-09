use crate::solution::Solution;

pub trait Algorithm {
    fn apply(&self, solution: &mut Solution);
}

pub struct Nil {}
impl Algorithm for Nil {
    fn apply(&self, _: &mut Solution) {}
}
