use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator clamping results of the underlying generator to a given interval.
///
/// For details, see the documentation of [`clamp()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`clamp()`] implemented by [`Generator`], should be used
/// to create [`Clamp`].
///
/// [`clamp()`]: Generator::clamp
#[derive(Clone, Copy, Debug)]
pub struct Clamp<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    min: f64,
    max: f64,
}

impl<G: Generator<1>> Generator1D for Clamp<1, G> {}
impl<G: Generator<2>> Generator2D for Clamp<2, G> {}
impl<G: Generator<3>> Generator3D for Clamp<3, G> {}
impl<G: Generator<4>> Generator4D for Clamp<4, G> {}

impl<const D: usize, G> Clamp<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, min: f64, max: f64) -> Self
    where
        G: Generator<D>,
    {
        Self {
            generator,
            min,
            max,
        }
    }
}

impl<const D: usize, G> Generator<D> for Clamp<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).clamp(self.min, self.max)
    }
}
