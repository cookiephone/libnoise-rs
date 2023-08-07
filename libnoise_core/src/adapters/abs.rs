use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Abs<G> {
    generator: G,
}

impl<G: Generator<1>> Generator1D for Abs<G> {}
impl<G: Generator<2>> Generator2D for Abs<G> {}
impl<G: Generator<3>> Generator3D for Abs<G> {}
impl<G: Generator<4>> Generator4D for Abs<G> {}

impl<G> Abs<G> {
    #[inline]
    pub fn new(generator: G) -> Self {
        Self { generator }
    }
}

impl<const D: usize, G> Generator<D> for Abs<G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).abs()
    }
}
