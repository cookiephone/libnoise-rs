use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Power<GA, GB> {
    generator_a: GA,
    generator_b: GB,
}

impl<GA: Generator<1>, GB: Generator<1>> Generator1D for Power<GA, GB> {}
impl<GA: Generator<2>, GB: Generator<2>> Generator2D for Power<GA, GB> {}
impl<GA: Generator<3>, GB: Generator<3>> Generator3D for Power<GA, GB> {}
impl<GA: Generator<4>, GB: Generator<4>> Generator4D for Power<GA, GB> {}

impl<GA, GB> Power<GA, GB> {
    #[inline]
    pub fn new(generator_a: GA, generator_b: GB) -> Self {
        Self {
            generator_a,
            generator_b,
        }
    }
}

impl<const D: usize, GA, GB> Generator<D> for Power<GA, GB>
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
