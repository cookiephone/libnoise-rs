use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::core::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

/// A generator which produces n-dimensional simplex noise.
///
/// For details, see the documentation of [`simplex()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`simplex()`] implemented by [`Source`], should be used to
/// create a simplex noise generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Simplex, Generator};
/// let generator = Simplex::new(42);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`simplex()`]: crate::Source::simplex
/// [`Source`]: crate::Source
#[derive(Clone, Debug)]
pub struct Simplex<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Simplex<1> {}
impl Generator2D for Simplex<2> {}
impl Generator3D for Simplex<3> {}
impl Generator4D for Simplex<4> {}

impl<const D: usize> Simplex<D> {
    /// Create a new simplex noise generator.
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
