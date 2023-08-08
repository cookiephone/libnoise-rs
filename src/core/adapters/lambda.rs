use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Lambda<const D: usize, G, L>
where
    G: Generator<D>,
    L: Fn(f64) -> f64,
{
    generator: G,
    lambda: L,
}

impl<G: Generator<1>, L: Fn(f64) -> f64 + Copy> Generator1D for Lambda<1, G, L> {}
impl<G: Generator<2>, L: Fn(f64) -> f64 + Copy> Generator2D for Lambda<2, G, L> {}
impl<G: Generator<3>, L: Fn(f64) -> f64 + Copy> Generator3D for Lambda<3, G, L> {}
impl<G: Generator<4>, L: Fn(f64) -> f64 + Copy> Generator4D for Lambda<4, G, L> {}

impl<const D: usize, G, L> Lambda<D, G, L>
where
    G: Generator<D>,
    L: Fn(f64) -> f64,
{
    #[inline]
    pub fn new(generator: G, lambda: L) -> Self {
        Self { generator, lambda }
    }
}

impl<const D: usize, G, L> Generator<D> for Lambda<D, G, L>
where
    G: Generator<D>,
    L: Copy + Fn(f64) -> f64,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        (self.lambda)(self.generator.sample(point))
    }
}
