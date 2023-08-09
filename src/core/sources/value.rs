use super::functional::{self, constants::PERMUTATION_TABLE_SIZE};
use crate::core::{
    generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D},
    utils::ptable::PermutationTable,
};

/// A generator which produces n-dimensional value noise.
///
/// For details, see the documentation of [`value()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`value()`] implemented by [`Source`], should be used to
/// create a value noise generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Value, Generator};
/// let generator = Value::new(42);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`value()`]: crate::Source::value
/// [`Source`]: crate::Source
#[derive(Clone, Debug)]
pub struct Value<const D: usize> {
    permutation_table: PermutationTable,
}

impl Generator1D for Value<1> {}
impl Generator2D for Value<2> {}
impl Generator3D for Value<3> {}
impl Generator4D for Value<4> {}

impl<const D: usize> Value<D> {
    /// Create a new value noise generator.
    #[inline]
    pub fn new(seed: u64) -> Self {
        let permutation_table = PermutationTable::new(seed, PERMUTATION_TABLE_SIZE, true);
        Self { permutation_table }
    }
}

impl Generator<1> for Value<1> {
    #[inline]
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::value::noise1d(&self.permutation_table, point)
    }
}

impl Generator<2> for Value<2> {
    #[inline]
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::value::noise2d(&self.permutation_table, point)
    }
}

impl Generator<3> for Value<3> {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::value::noise3d(&self.permutation_table, point)
    }
}

impl Generator<4> for Value<4> {
    #[inline]
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::value::noise4d(&self.permutation_table, point)
    }
}
