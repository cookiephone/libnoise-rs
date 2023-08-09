use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator which translates input points before passing them to the underlying generator.
///
/// For details, see the documentation of [`translate()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`translate()`] implemented by [`Generator`], should be used
/// to create [`Translate`].
///
/// [`translate()`]: Generator::translate
#[derive(Clone, Copy, Debug)]
pub struct Translate<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    translation: [f64; D],
}

impl<G: Generator<1>> Generator1D for Translate<1, G> {}
impl<G: Generator<2>> Generator2D for Translate<2, G> {}
impl<G: Generator<3>> Generator3D for Translate<3, G> {}
impl<G: Generator<4>> Generator4D for Translate<4, G> {}

impl<const D: usize, G> Translate<D, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, translation: [f64; D]) -> Self {
        Self {
            generator,
            translation,
        }
    }
}

impl<const D: usize, G> Generator<D> for Translate<D, G>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator
            .sample(std::array::from_fn(|i| point[i] + self.translation[i]))
    }
}
