use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Clamp<G> {
    generator: G,
    min: f64,
    max: f64,
}

impl<G: Generator<1>> Generator1D for Clamp<G> {}
impl<G: Generator<2>> Generator2D for Clamp<G> {}
impl<G: Generator<3>> Generator3D for Clamp<G> {}
impl<G: Generator<4>> Generator4D for Clamp<G> {}

impl<G> Clamp<G> {
    pub fn new(generator: G, min: f64, max: f64) -> Self {
        Self {
            generator,
            min,
            max,
        }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Clamp<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).clamp(self.min, self.max)
    }
}
