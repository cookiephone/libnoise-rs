use super::functional;
use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator which produces an n-dimensional checkerboard pattern.
///
/// For details, see the documentation of [`checkerboard()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`checkerboard()`] implemented by [`Source`], should be used to
/// create a checkerboard generator.
///
/// # Direct usage of this struct
///
/// [`Checkerboard`] is a unit struct and thus can be used directly:
///
/// ```
/// # use libnoise::{Checkerboard, Generator};
/// let value = Checkerboard.sample([0.2, 0.5]);
/// ```
///
/// Alternatively, for the sake of a unified API, the function [`new()`] is provided:
///
/// ```
/// // create
/// # use libnoise::{Checkerboard, Generator};
/// let generator = Checkerboard::new();
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`checkerboard()`]: crate::Source::checkerboard
/// [`Source`]: crate::Source
/// [`new()`]: Checkerboard::new
#[derive(Clone, Copy, Debug)]
pub struct Checkerboard<const D: usize>;

impl Generator1D for Checkerboard<1> {}
impl Generator2D for Checkerboard<2> {}
impl Generator3D for Checkerboard<3> {}
impl Generator4D for Checkerboard<4> {}

#[allow(clippy::new_without_default)]
impl<const D: usize> Checkerboard<D> {
    /// Create a new checkerboard generator.
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Generator<1> for Checkerboard<1> {
    #[inline]
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::checkerboard::noise1d(point)
    }
}

impl Generator<2> for Checkerboard<2> {
    #[inline]
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::checkerboard::noise2d(point)
    }
}

impl Generator<3> for Checkerboard<3> {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::checkerboard::noise3d(point)
    }
}

impl Generator<4> for Checkerboard<4> {
    #[inline]
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::checkerboard::noise4d(point)
    }
}
