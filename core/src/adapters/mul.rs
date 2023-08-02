use crate::generator::Generator;

pub struct Mul<const D: usize, G: Generator<D>> {
    generator: G,
    scale: f64,
}

impl<const D: usize, G: Generator<D>> Mul<D, G> {
    pub fn new(generator: G, scale: f64) -> Self {
        Self { generator, scale }
    }
}

macro_rules! impl_generator {
    ($dim:literal, $target:ident) => {
        impl<G: Generator<$dim>> Generator<$dim> for $target<$dim, G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                self.generator.sample(point) * self.scale
            }
        }
    };
}

impl_generator!(1, Mul);
impl_generator!(2, Mul);
impl_generator!(3, Mul);
impl_generator!(4, Mul);
