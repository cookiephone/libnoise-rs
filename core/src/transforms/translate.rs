use crate::generator::Generator;

pub struct Translate<const D: usize, G: Generator<D>> {
    generator: G,
    translation: [f64; D],
}

impl<const D: usize, G: Generator<D>> Translate<D, G> {
    pub fn new(generator: G, translation: [f64; D]) -> Self {
        Self {
            generator,
            translation,
        }
    }
}

impl<G: Generator<1>> Generator<1> for Translate<1, G> {
    fn sample(&self, point: [f64; 1]) -> f64 {
        self.generator.sample([point[0] + self.translation[0]])
    }
}

impl<G: Generator<2>> Generator<2> for Translate<2, G> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        self.generator.sample([
            point[0] + self.translation[0],
            point[1] + self.translation[1],
        ])
    }
}

impl<G: Generator<3>> Generator<3> for Translate<3, G> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        self.generator.sample([
            point[0] + self.translation[0],
            point[1] + self.translation[1],
            point[2] + self.translation[2],
        ])
    }
}

impl<G: Generator<4>> Generator<4> for Translate<4, G> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        self.generator.sample([
            point[0] + self.translation[0],
            point[1] + self.translation[1],
            point[2] + self.translation[2],
            point[3] + self.translation[3],
        ])
    }
}
