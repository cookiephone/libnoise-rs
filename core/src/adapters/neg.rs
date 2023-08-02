use crate::generator::Generator;

#[derive(Clone)]
pub struct Neg<G> {
    generator: G,
}

impl<G> Neg<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Neg<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        -self.generator.sample(point)
    }
}
