use crate::generator::Generator;

#[derive(Clone)]
pub struct Scale<const D: usize, G> {
    generator: G,
    scale: [f64; D],
}

impl<const D: usize, G> Scale<D, G> {
    pub fn new(generator: G, scale: [f64; D]) -> Self {
        Self { generator, scale }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Scale<D, G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator
            .sample(std::array::from_fn(|i| point[i] * self.scale[i]))
    }
}
