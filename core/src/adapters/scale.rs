use crate::generator::Generator;

pub struct Scale<const D: usize, G: Generator<D>> {
    generator: G,
    scale: [f64; D],
}

impl<const D: usize, G: Generator<D>> Scale<D, G> {
    pub fn new(generator: G, scale: [f64; D]) -> Self {
        Self { generator, scale }
    }
}

impl<G: Generator<1>> Generator<1> for Scale<1, G> {
    fn sample(&self, point: [f64; 1]) -> f64 {
        self.generator.sample([point[0] * self.scale[0]])
    }
}

impl<G: Generator<2>> Generator<2> for Scale<2, G> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        self.generator
            .sample([point[0] * self.scale[0], point[1] * self.scale[1]])
    }
}

impl<G: Generator<3>> Generator<3> for Scale<3, G> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        self.generator.sample([
            point[0] * self.scale[0],
            point[1] * self.scale[1],
            point[2] * self.scale[2],
        ])
    }
}

impl<G: Generator<4>> Generator<4> for Scale<4, G> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        self.generator.sample([
            point[0] * self.scale[0],
            point[1] * self.scale[1],
            point[2] * self.scale[2],
            point[3] * self.scale[3],
        ])
    }
}
