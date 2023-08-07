use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Pow<G, T> {
    generator: G,
    exponent: T,
}

impl<G: Generator<1>> Generator1D for Pow<G, i32> {}
impl<G: Generator<2>> Generator2D for Pow<G, i32> {}
impl<G: Generator<3>> Generator3D for Pow<G, i32> {}
impl<G: Generator<4>> Generator4D for Pow<G, i32> {}

impl<G: Generator<1>> Generator1D for Pow<G, f64> {}
impl<G: Generator<2>> Generator2D for Pow<G, f64> {}
impl<G: Generator<3>> Generator3D for Pow<G, f64> {}
impl<G: Generator<4>> Generator4D for Pow<G, f64> {}

impl<G, T> Pow<G, T> {
    #[inline]
    pub fn new(generator: G, exponent: T) -> Self {
        Self {
            generator,
            exponent,
        }
    }
}

impl<const D: usize, G> Generator<D> for Pow<G, i32>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).powi(self.exponent)
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Pow<G, f64>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).powf(self.exponent)
    }
}
