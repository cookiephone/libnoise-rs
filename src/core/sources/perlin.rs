use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::core::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

#[derive(Clone)]
pub struct Perlin<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Perlin<1> {}
impl Generator2D for Perlin<2> {}
impl Generator3D for Perlin<3> {}
impl Generator4D for Perlin<4> {}

impl<const D: usize> Perlin<D> {
    #[inline]
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Perlin<1> {
    #[inline]
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::perlin::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Perlin<2> {
    #[inline]
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::perlin::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Perlin<3> {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::perlin::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Perlin<4> {
    #[inline]
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::perlin::noise4d(&self.permutation_table, point)
    }
}
