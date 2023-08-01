use crate::generator::Generator;

pub struct Add<const D: usize, G: Generator<D>> {
    generator: G,
    offset: f64,
}

impl<const D: usize, G: Generator<D>> Add<D, G> {
    pub fn new(generator: G, offset: f64) -> Self {
        Self { generator, offset }
    }
}

macro_rules! impl_generator {
    ($dim:literal, $target:ident) => {
        impl<G: Generator<$dim>> Generator<$dim> for $target<$dim, G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                self.generator.sample(point) + self.offset
            }
        }
    };
}

impl_generator!(1, Add);
impl_generator!(2, Add);
impl_generator!(3, Add);
impl_generator!(4, Add);
