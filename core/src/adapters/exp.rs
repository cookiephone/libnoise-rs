use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Exp<G> {
    generator: G,
}

impl<G: Generator<1>> Generator1D for Exp<G> {}
impl<G: Generator<2>> Generator2D for Exp<G> {}
impl<G: Generator<3>> Generator3D for Exp<G> {}
impl<G: Generator<4>> Generator4D for Exp<G> {}

impl<G> Exp<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Exp<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).exp()
    }
}
