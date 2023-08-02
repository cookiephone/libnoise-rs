use crate::generator::Generator;

#[derive(Clone)]
pub struct Add<G> {
    generator: G,
    offset: f64,
}

impl<G> Add<G> {
    pub fn new(generator: G, offset: f64) -> Self {
        Self { generator, offset }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Add<G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point) + self.offset
    }
}
