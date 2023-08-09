use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator multiplying results of the underlying generator to results of a given other
/// generator.
///
/// For details, see the documentation of [`product()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`product()`] implemented by [`Generator`], should be used
/// to create [`Product`].
///
/// [`product()`]: Generator::product
#[derive(Clone, Copy, Debug)]
pub struct Product<const D: usize, GA, GB> {
    generator_a: GA,
    generator_b: GB,
}

impl<GA: Generator<1>, GB: Generator<1>> Generator1D for Product<1, GA, GB> {}
impl<GA: Generator<2>, GB: Generator<2>> Generator2D for Product<2, GA, GB> {}
impl<GA: Generator<3>, GB: Generator<3>> Generator3D for Product<3, GA, GB> {}
impl<GA: Generator<4>, GB: Generator<4>> Generator4D for Product<4, GA, GB> {}

impl<const D: usize, GA, GB> Product<D, GA, GB>
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

impl<const D: usize, GA, GB> Generator<D> for Product<D, GA, GB>
where
    GA: Generator<D>,
    GB: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator_a.sample(point) * self.generator_b.sample(point)
    }
}
