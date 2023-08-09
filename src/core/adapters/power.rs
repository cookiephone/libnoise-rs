use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator raising results of the underlying generator to the power of results of a
/// given other generator.
///
/// For details, see the documentation of [`power()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`power()`] implemented by [`Generator`], should be used
/// to create [`Power`].
///
/// [`power()`]: Generator::power
#[derive(Clone, Copy, Debug)]
pub struct Power<const D: usize, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    generator_a: GA,
    generator_b: GB,
}

impl<GA: Generator<1>, GB: Generator<1>> Generator1D for Power<1, GA, GB> {}
impl<GA: Generator<2>, GB: Generator<2>> Generator2D for Power<2, GA, GB> {}
impl<GA: Generator<3>, GB: Generator<3>> Generator3D for Power<3, GA, GB> {}
impl<GA: Generator<4>, GB: Generator<4>> Generator4D for Power<4, GA, GB> {}

impl<const D: usize, GA, GB> Power<D, GA, GB>
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

impl<const D: usize, GA, GB> Generator<D> for Power<D, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator_a
            .sample(point)
            .powf(self.generator_b.sample(point))
    }
}
