use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::{generator::Generator, utils::ptable::PermutationTable};

#[derive(Clone)]
pub struct Value<const D: usize> {
    permutation_table: PermutationTable,
}

impl<const D: usize> Value<D> {
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Value<1> {
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::value::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Value<2> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::value::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Value<3> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::value::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Value<4> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::value::noise4d(&self.permutation_table, point)
    }
}
