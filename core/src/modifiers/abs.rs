use crate::generator::Generator;

pub struct Abs<const D: usize, G: Generator<D>> {
    generator: G,
}

impl<const D: usize, G: Generator<D>> Abs<D, G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

macro_rules! impl_generator {
    ($dim:literal, $target:ident) => {
        impl<G: Generator<$dim>> Generator<$dim> for $target<$dim, G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                self.generator.sample(point).abs()
            }
        }
    };
}

impl_generator!(1, Abs);
impl_generator!(2, Abs);
impl_generator!(3, Abs);
impl_generator!(4, Abs);
