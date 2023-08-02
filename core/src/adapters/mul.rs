use crate::generator::Generator;

#[derive(Clone)]
pub struct Mul<G> {
    generator: G,
    scale: f64,
}

impl<G> Mul<G> {
    pub fn new(generator: G, scale: f64) -> Self {
        Self { generator, scale }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Mul<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point) * self.scale
    }
}
