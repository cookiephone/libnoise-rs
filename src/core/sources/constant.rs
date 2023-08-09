use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator which produces the supplied value for every input point.
///
/// For details, see the documentation of [`constant()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`constant()`] implemented by [`Source`], should be used to
/// create a constant generator.
///
/// # Direct usage of this struct
///
/// Direct instantiation of this struct:
///
/// ```
/// # use libnoise::{Constant, Generator};
/// let generator = Constant::new(6.9);
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// [`constant()`]: crate::Source::constant
/// [`Source`]: crate::Source
#[derive(Clone, Copy, Debug)]
pub struct Constant<const D: usize> {
    value: f64,
}

impl Generator1D for Constant<1> {}
impl Generator2D for Constant<2> {}
impl Generator3D for Constant<3> {}
impl Generator4D for Constant<4> {}

impl<const D: usize> Constant<D> {
    /// Create a new constant generator.
    #[inline]
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl<const D: usize> Generator<D> for Constant<D> {
    #[inline]
    fn sample(&self, _point: [f64; D]) -> f64 {
        self.value
    }
}
