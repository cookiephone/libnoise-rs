use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::core::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

/// A generator which produces n-dimensional worley noise.
///
/// For details, see the documentation of [`worley()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`worley()`] implemented by [`Source`], should be used to
/// create a worley noise generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Worley, Generator};
/// let generator = Worley::new(42);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`worley()`]: crate::Source::worley
/// [`Source`]: crate::Source
#[derive(Clone, Debug)]
pub struct Worley<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Worley<1> {}
impl Generator2D for Worley<2> {}
impl Generator3D for Worley<3> {}
impl Generator4D for Worley<4> {}

impl<const D: usize> Worley<D> {
    /// Create a new worley noise generator.
    #[inline]
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Worley<1> {
    #[inline]
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::worley::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Worley<2> {
    #[inline]
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::worley::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Worley<3> {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::worley::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Worley<4> {
    #[inline]
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::worley::noise4d(&self.permutation_table, point)
    }
}
