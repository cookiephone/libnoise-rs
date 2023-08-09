use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator raising results of the underlying generator to the power of `exponent`.
///
/// For details, see the documentation of [`powi()`] and [`powf()`]. Typically, this
/// struct is not meant to be used directly. Instead, [`powi()`] or [`powf()`]
/// implemented by [`Generator`], should be used to create [`Pow`].
///
/// [`powi()`]: Generator::powi
/// [`powf()`]: Generator::powf
#[derive(Clone, Copy, Debug)]
pub struct Pow<const D: usize, G, T>
where
    G: Generator<D>,
{
    generator: G,
    exponent: T,
}

impl<G: Generator<1>> Generator1D for Pow<1, G, i32> {}
impl<G: Generator<2>> Generator2D for Pow<2, G, i32> {}
impl<G: Generator<3>> Generator3D for Pow<3, G, i32> {}
impl<G: Generator<4>> Generator4D for Pow<4, G, i32> {}

impl<G: Generator<1>> Generator1D for Pow<1, G, f64> {}
impl<G: Generator<2>> Generator2D for Pow<2, G, f64> {}
impl<G: Generator<3>> Generator3D for Pow<3, G, f64> {}
impl<G: Generator<4>> Generator4D for Pow<4, G, f64> {}

impl<const D: usize, G, T> Pow<D, G, T>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, exponent: T) -> Self {
        Self {
            generator,
            exponent,
        }
    }
}

impl<const D: usize, G> Generator<D> for Pow<D, G, i32>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).powi(self.exponent)
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Pow<D, G, f64>
where
    G: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.generator.sample(point).powf(self.exponent)
    }
}
