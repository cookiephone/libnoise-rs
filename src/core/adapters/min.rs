use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator producing the minimum of results of the underlying generator and results of
/// a given other generator.
///
/// For details, see the documentation of [`min()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`min()`] implemented by [`Generator`], should be used
/// to create [`Min`].
///
/// [`min()`]: Generator::min
#[derive(Clone, Copy, Debug)]
pub struct Min<const D: usize, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    generator_a: GA,
    generator_b: GB,
}

impl<GA: Generator<1>, GB: Generator<1>> Generator1D for Min<1, GA, GB> {}
impl<GA: Generator<2>, GB: Generator<2>> Generator2D for Min<2, GA, GB> {}
impl<GA: Generator<3>, GB: Generator<3>> Generator3D for Min<3, GA, GB> {}
impl<GA: Generator<4>, GB: Generator<4>> Generator4D for Min<4, GA, GB> {}

impl<const D: usize, GA, GB> Min<D, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    #[inline]
    pub fn new(generator_a: GA, generator_b: GB) -> Self {
        Self {
            generator_a,
            generator_b,
        }
    }
}

impl<const D: usize, GA, GB> Generator<D> for Min<D, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator_a
            .sample(point)
            .min(self.generator_b.sample(point))
    }
}
