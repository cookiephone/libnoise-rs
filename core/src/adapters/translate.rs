use crate::generator::Generator;

#[derive(Clone)]
pub struct Translate<const D: usize, G> {
    generator: G,
    translation: [f64; D],
}

impl<const D: usize, G> Translate<D, G> {
    pub fn new(generator: G, translation: [f64; D]) -> Self {
        Self {
            generator,
            translation,
        }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Translate<D, G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator
            .sample(std::array::from_fn(|i| point[i] + self.translation[i]))
    }
}
