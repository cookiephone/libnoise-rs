use crate::generator::Generator;

#[derive(Clone)]
pub struct Clamp<G> {
    generator: G,
    min: f64,
    max: f64,
}

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
