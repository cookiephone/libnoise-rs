use crate::generator::Generator;

#[derive(Clone)]
pub struct Lambda<G, L> {
    generator: G,
    lambda: L,
}

impl<G, L> Lambda<G, L> {
    pub fn new(generator: G, lambda: L) -> Self {
        Self { generator, lambda }
    }
}

impl<const D: usize, G: Generator<D>, L: Copy + Fn(f64) -> f64> Generator<D> for Lambda<G, L> {
    fn sample(&self, point: [f64; D]) -> f64 {
        (self.lambda)(self.generator.sample(point))
    }
}
