use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Lambda<G, L> {
    generator: G,
    lambda: L,
}

impl<G: Generator<1>, L: Fn(f64) -> f64 + Copy> Generator1D for Lambda<G, L> {}
impl<G: Generator<2>, L: Fn(f64) -> f64 + Copy> Generator2D for Lambda<G, L> {}
impl<G: Generator<3>, L: Fn(f64) -> f64 + Copy> Generator3D for Lambda<G, L> {}
impl<G: Generator<4>, L: Fn(f64) -> f64 + Copy> Generator4D for Lambda<G, L> {}

impl<G, L> Lambda<G, L> {
    #[inline]
    pub fn new(generator: G, lambda: L) -> Self {
        Self { generator, lambda }
    }
}

impl<const D: usize, G, L> Generator<D> for Lambda<G, L>
where
    G: Generator<D>,
    L: Copy + Fn(f64) -> f64,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        (self.lambda)(self.generator.sample(point))
    }
}
