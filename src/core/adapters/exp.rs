use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator applying the exponential function on results of the underlying generator.
///
/// For details, see the documentation of [`exp()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`exp()`] implemented by [`Generator`], should be used
/// to create [`Exp`].
///
/// [`exp()`]: Generator::exp
#[derive(Clone, Copy, Debug)]
pub struct Exp<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
}

impl<G: Generator<1>> Generator1D for Exp<1, G> {}
impl<G: Generator<2>> Generator2D for Exp<2, G> {}
impl<G: Generator<3>> Generator3D for Exp<3, G> {}
impl<G: Generator<4>> Generator4D for Exp<4, G> {}

impl<const D: usize, G> Exp<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G> Generator<D> for Exp<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).exp()
    }
}
