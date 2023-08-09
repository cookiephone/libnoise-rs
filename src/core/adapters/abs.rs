use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator returning the absolute value of the results of the underlying generator.
///
/// For details, see the documentation of [`abs()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`abs()`] implemented by [`Generator`], should be used
/// to create [`Abs`].
///
/// [`abs()`]: Generator::abs
#[derive(Clone, Copy, Debug)]
pub struct Abs<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
}

impl<G: Generator<1>> Generator1D for Abs<1, G> {}
impl<G: Generator<2>> Generator2D for Abs<2, G> {}
impl<G: Generator<3>> Generator3D for Abs<3, G> {}
impl<G: Generator<4>> Generator4D for Abs<4, G> {}

impl<const D: usize, G> Abs<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G> Generator<D> for Abs<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).abs()
    }
}
