use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::{generator::Generator, utils::ptable::PermutationTable};

pub struct Simplex {
    permutation_table: PermutationTable,
}

impl Simplex {
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Simplex {
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::simplex::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Simplex {
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::simplex::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Simplex {
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::simplex::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Simplex {
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::simplex::noise4d(&self.permutation_table, point)
    }
}

pub fn simplex(seed: u64) -> Simplex {
    Simplex::new(seed)
}
