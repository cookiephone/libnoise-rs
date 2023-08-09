use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator which scales input points before passing them to the underlying generator.
///
/// For details, see the documentation of [`scale()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`scale()`] implemented by [`Generator`], should be used
/// to create [`Scale`].
///
/// [`scale()`]: Generator::scale
#[derive(Clone, Copy, Debug)]
pub struct Scale<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    scale: [f64; D],
}

impl<G: Generator<1>> Generator1D for Scale<1, G> {}
impl<G: Generator<2>> Generator2D for Scale<2, G> {}
impl<G: Generator<3>> Generator3D for Scale<3, G> {}
impl<G: Generator<4>> Generator4D for Scale<4, G> {}

impl<const D: usize, G> Scale<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, scale: [f64; D]) -> Self {
        Self { generator, scale }
    }
}

impl<const D: usize, G> Generator<D> for Scale<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator
            .sample(std::array::from_fn(|i| point[i] * self.scale[i]))
    }
}
