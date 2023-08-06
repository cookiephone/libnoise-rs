use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Neg<G> {
    generator: G,
}

impl<G: Generator<1>> Generator1D for Neg<G> {}
impl<G: Generator<2>> Generator2D for Neg<G> {}
impl<G: Generator<3>> Generator3D for Neg<G> {}
impl<G: Generator<4>> Generator4D for Neg<G> {}

impl<G> Neg<G> {
    #[inline]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Neg<G> {
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        -self.generator.sample(point)
    }
}
