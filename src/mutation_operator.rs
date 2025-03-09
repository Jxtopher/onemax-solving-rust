use globals::GLOBALS;
use rand::Rng;

use crate::globals;
use crate::solution::Solution;

// Each bit in a solution and changes the value of the bit with probability p
pub fn bitflip(solution: &mut Solution, p: f64) {
    let mut globals = GLOBALS.lock().unwrap();

    for index in 0..solution.get_size() {
        let random_float: f64 = globals.rng.random();
        if random_float < p {
            let old_value = solution.get_bit(index);
            let new_value = !old_value;
            solution.set_change(index, new_value);
        }
    }
}

// Chooses uniformly at random k bits in the current solution and flips their values
// pub fn kbit(solution: &mut Solution, k: u64) {}
