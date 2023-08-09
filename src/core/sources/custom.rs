use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator which produces n-dimensional values based on the provided closure.
///
/// For details, see the documentation of [`custom()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`custom()`] implemented by [`Source`], should be used to
/// create a custom generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Custom, Generator};
/// let generator = Custom::new(|[x, y]| x % 2.0 + (1.0 - y * y) % 3.0);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`custom()`]: crate::Source::custom
/// [`Source`]: crate::Source
#[derive(Clone, Copy, Debug)]
pub struct Custom<const D: usize, N> {
    noise: N,
}

impl<N: Fn([f64; 1]) -> f64> Generator1D for Custom<1, N> {}
impl<N: Fn([f64; 2]) -> f64> Generator2D for Custom<2, N> {}
impl<N: Fn([f64; 3]) -> f64> Generator3D for Custom<3, N> {}
impl<N: Fn([f64; 4]) -> f64> Generator4D for Custom<4, N> {}

impl<const D: usize, N: Fn([f64; D]) -> f64> Custom<D, N> {
    /// Create a new constant generator.
    #[inline]
    pub fn new(noise: N) -> Self {
        Self { noise }
    }
}

impl<const D: usize, N: Fn([f64; D]) -> f64> Generator<D> for Custom<D, N> {
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        (self.noise)(point)
    }
}
