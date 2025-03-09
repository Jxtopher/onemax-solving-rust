use globals::GLOBALS;
use rand::Rng;
use std::collections::HashSet;

use crate::algorithm::Algorithm;
use crate::globals;
use crate::solution::Solution;

// Each bit in a solution and changes the value of the bit with probability p
pub struct Bitflip {
    pub p: f64,
}

impl Algorithm for Bitflip {
    fn apply(&self, solution: &mut Solution) {
        let mut globals = GLOBALS.lock().unwrap();

        for index in 0..solution.get_size() {
            let random_float: f64 = globals.rng.random();
            if random_float < self.p {
                let old_value = solution.get_bit(index);
                let new_value = !old_value;
                solution.set_change(index, new_value);
            }
        }
    }
}

// Chooses uniformly at random k bits in the current solution and flips their values
pub struct Kbit {
    pub k: usize,
}

impl Algorithm for Kbit {
    fn apply(&self, solution: &mut Solution) {
        let mut globals = GLOBALS.lock().unwrap();

        // Generate k distinct random numbers between 0 and 100
        let mut random_numbers = HashSet::new();
        while random_numbers.len() < self.k {
            let num = globals.rng.random_range(0..solution.get_size());
            random_numbers.insert(num);
        }

        // Mute k bits
        for random_number in random_numbers {
            let old_value = solution.get_bit(random_number);
            let new_value = !old_value;
            solution.set_change(random_number, new_value);
        }
    }
}
