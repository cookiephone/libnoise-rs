use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator adding `offset` to results of the underlying generator.
///
/// For details, see the documentation of [`add()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`add()`] implemented by [`Generator`], should be used
/// to create [`Add`].
///
/// [`add()`]: Generator::add
#[derive(Clone, Copy, Debug)]
pub struct Add<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    offset: f64,
}

impl<G: Generator<1>> Generator1D for Add<1, G> {}
impl<G: Generator<2>> Generator2D for Add<2, G> {}
impl<G: Generator<3>> Generator3D for Add<3, G> {}
impl<G: Generator<4>> Generator4D for Add<4, G> {}

impl<const D: usize, G> Add<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, offset: f64) -> Self {
        Self { generator, offset }
    }
}

impl<const D: usize, G> Generator<D> for Add<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point) + self.offset
    }
}
