use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::{generator::Generator, utils::ptable::PermutationTable};

#[derive(Clone)]
pub struct Value {
    permutation_table: PermutationTable,
}

impl Value {
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Value {
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::value::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Value {
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::value::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Value {
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::value::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Value {
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::value::noise4d(&self.permutation_table, point)
    }
}

pub fn value(seed: u64) -> Value {
    Value::new(seed)
}
