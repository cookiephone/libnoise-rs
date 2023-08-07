use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Add<G> {
    generator: G,
    offset: f64,
}

impl<G: Generator<1>> Generator1D for Add<G> {}
impl<G: Generator<2>> Generator2D for Add<G> {}
impl<G: Generator<3>> Generator3D for Add<G> {}
impl<G: Generator<4>> Generator4D for Add<G> {}

impl<G> Add<G> {
    #[inline]
    pub fn new(generator: G, offset: f64) -> Self {
        Self { generator, offset }
    }
}

impl<const D: usize, G> Generator<D> for Add<G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point) + self.offset
    }
}
