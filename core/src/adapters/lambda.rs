use crate::generator::Generator;

pub struct Lambda<const D: usize, G: Generator<D>, L: Fn(f64) -> f64> {
    generator: G,
    lambda: L,
}

impl<const D: usize, G: Generator<D>, L: Fn(f64) -> f64> Lambda<D, G, L> {
    pub fn new(generator: G, lambda: L) -> Self {
        Self { generator, lambda }
    }
}

macro_rules! impl_generator {
    ($dim:literal, $target:ident) => {
        impl<G: Generator<$dim>, L: Fn(f64) -> f64> Generator<$dim> for $target<$dim, G, L> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                (self.lambda)(self.generator.sample(point))
            }
        }
    };
}

impl_generator!(1, Lambda);
impl_generator!(2, Lambda);
impl_generator!(3, Lambda);
impl_generator!(4, Lambda);
