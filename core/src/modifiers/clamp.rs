use crate::generator::Generator;

pub struct Clamp<const D: usize, G: Generator<D>> {
    generator: G,
    min: f64,
    max: f64,
}

impl<const D: usize, G: Generator<D>> Clamp<D, G> {
    pub fn new(generator: G, min: f64, max: f64) -> Self {
        Self {
            generator,
            min,
            max,
        }
    }
}

macro_rules! impl_generator {
    ($dim:literal, $target:ident) => {
        impl<G: Generator<$dim>> Generator<$dim> for $target<$dim, G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                self.generator.sample(point).clamp(self.min, self.max)
            }
        }
    };
}

impl_generator!(1, Clamp);
impl_generator!(2, Clamp);
impl_generator!(3, Clamp);
impl_generator!(4, Clamp);
