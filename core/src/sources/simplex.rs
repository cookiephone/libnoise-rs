use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

#[derive(Clone)]
pub struct Simplex<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Simplex<1> {}
impl Generator2D for Simplex<2> {}
impl Generator3D for Simplex<3> {}
impl Generator4D for Simplex<4> {}

impl<const D: usize> Simplex<D> {
    #[inline]
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Simplex<1> {
    #[inline]
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::simplex::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Simplex<2> {
    #[inline]
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::simplex::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Simplex<3> {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::simplex::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Simplex<4> {
    #[inline]
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::simplex::noise4d(&self.permutation_table, point)
    }
}
