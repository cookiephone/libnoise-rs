use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator multiplying `scale` to results of the underlying generator.
///
/// For details, see the documentation of [`mul()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`mul()`] implemented by [`Generator`], should be used
/// to create [`Mul`].
///
/// [`mul()`]: Generator::mul
#[derive(Clone, Copy, Debug)]
pub struct Mul<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    scale: f64,
}

impl<G: Generator<1>> Generator1D for Mul<1, G> {}
impl<G: Generator<2>> Generator2D for Mul<2, G> {}
impl<G: Generator<3>> Generator3D for Mul<3, G> {}
impl<G: Generator<4>> Generator4D for Mul<4, G> {}

impl<const D: usize, G> Mul<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, scale: f64) -> Self {
        Self { generator, scale }
    }
}

impl<const D: usize, G> Generator<D> for Mul<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point) * self.scale
    }
}
