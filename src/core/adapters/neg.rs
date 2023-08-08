use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Neg<const D: usize, G> {
    generator: G,
}

impl<G: Generator<1>> Generator1D for Neg<1, G> {}
impl<G: Generator<2>> Generator2D for Neg<2, G> {}
impl<G: Generator<3>> Generator3D for Neg<3, G> {}
impl<G: Generator<4>> Generator4D for Neg<4, G> {}

impl<const D: usize, G> Neg<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G> Generator<D> for Neg<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        -self.generator.sample(point)
    }
}
