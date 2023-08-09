use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::core::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

/// A generator which produces n-dimensional perlin noise.
///
/// For details, see the documentation of [`perlin()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`perlin()`] implemented by [`Source`], should be used to
/// create a perlin noise generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Perlin, Generator};
/// let generator = Perlin::new(42);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`perlin()`]: crate::Source::perlin
/// [`Source`]: crate::Source
#[derive(Clone, Debug)]
pub struct Perlin<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Perlin<1> {}
impl Generator2D for Perlin<2> {}
impl Generator3D for Perlin<3> {}
impl Generator4D for Perlin<4> {}

impl<const D: usize> Perlin<D> {
    /// Create a new perlin noise generator.
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
