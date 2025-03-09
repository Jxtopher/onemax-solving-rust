use bitvec::prelude::*;
use std::fmt;

use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
pub struct Solution {
    bit_vec: BitVec<u8, Lsb0>,
    fitness: i64,
    is_valid: bool,
    changes: Vec<(usize, bool)>,
    changes_fitness: i64,
}

impl Solution {
    pub fn new(size: usize) -> Self {
        Self {
            bit_vec: bitvec![u8, Lsb0; 0; size],
            fitness: 0,
            is_valid: true,
            changes: Vec::new(),
            changes_fitness: 0,
        }
    }
    pub fn get_size(&self) -> usize {
        self.bit_vec.len()
    }
    pub fn set_fitness(&mut self, value: i64) {
        self.fitness = value;
        self.is_valid = true;
    }
    pub fn get_fitness(&self) -> i64 {
        self.fitness
    }
    pub fn set_bit(&mut self, index: usize, value: bool) {
        if self.get_bit(index) != value {
            self.bit_vec.set(index, value);
            self.is_valid = false;
        }
    }
    pub fn get_bit(&self, index: usize) -> bool {
        //self.bit_vec.get(index).map_or(false, |bit| *bit)
        self.bit_vec.get(index).is_some_and(|bit| *bit)
    }
    pub fn get_is_valid(&self) -> bool {
        self.is_valid
    }
    pub fn set_change(&mut self, index: usize, value: bool) {
        let current_bit = self.get_bit(index);

        if current_bit && !value {
            self.changes_fitness -= 1;
        } else if !current_bit && value {
            self.changes_fitness += 1;
        }

        self.changes.push((index, value));
    }
    pub fn get_change_fitness(&self) -> i64 {
        self.changes_fitness
    }
    pub fn commit(&mut self) {
        for (index, value) in self.changes.iter() {
            if self.get_bit(*index) != *value {
                self.bit_vec.set(*index, *value);
                self.is_valid = false;
            }
        }
        self.fitness = self.changes_fitness;
    }
    pub fn revert_changes(&mut self) {
        self.changes_fitness = self.fitness;
        self.changes.clear();
    }

    pub fn reset(&mut self) {
        for i in 0..self.bit_vec.len() {
            self.bit_vec.set(i, false);
        }
        self.fitness = 0;
        self.changes_fitness = 0;
        self.is_valid = true;
        self.changes.clear();
    }
}

// Implement the Display trait for Car
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.fitness)?;

        for bit in self.bit_vec.iter() {
            write!(f, "{}", if *bit { "1" } else { "0" })?;
        }

        Ok(())
    }
}

impl Clone for Solution {
    fn clone(&self) -> Solution {
        Solution {
            bit_vec: self.bit_vec.clone(),
            fitness: self.fitness,
            is_valid: self.is_valid,
            changes: self.changes.clone(),
            changes_fitness: self.changes_fitness,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitness() {
        let mut s = Solution::new(0);
        assert_eq!(s.get_fitness(), 0);

        s.set_fitness(42);
        assert_eq!(s.get_fitness(), 42);
    }

    #[test]
    fn is_valid() {
        let mut s = Solution::new(5);

        assert!(s.get_is_valid());

        s.set_bit(2, true);
        assert!(!s.get_bit(0));
        assert!(!s.get_bit(1));
        assert!(s.get_bit(2));
        assert!(!s.get_bit(3));
        assert!(!s.get_bit(4));

        s.set_bit(2, false);
        assert!(s.get_bit(2));
    }

    #[test]
    fn commit() {
        let mut s = Solution::new(10);
        s.set_change(2, true);
        s.set_change(5, true);
        s.set_change(6, false);

        assert_eq!(s.get_fitness(), 0);
        assert_eq!(s.get_change_fitness(), 2);
        s.commit();
        assert_eq!(s.get_fitness(), 2);
        assert_eq!(s.get_change_fitness(), 2);
    }

    #[test]
    fn revert() {
        let mut s = Solution::new(10);
        s.set_change(2, true);
        s.set_change(5, true);
        s.set_change(6, false);

        assert_eq!(s.get_fitness(), 0);
        assert_eq!(s.get_change_fitness(), 2);
        s.revert_changes();
        assert_eq!(s.get_fitness(), 0);
        assert_eq!(s.get_change_fitness(), 0);
    }
}
