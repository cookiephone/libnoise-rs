use crate::generator::Generator;

#[derive(Clone)]
pub struct Abs<G> {
    generator: G,
}

impl<G> Abs<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Abs<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).abs()
    }
}
