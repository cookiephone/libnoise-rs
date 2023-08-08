use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Custom<const D: usize, N> {
    noise: N,
}

impl<N: Fn([f64; 1]) -> f64> Generator1D for Custom<1, N> {}
impl<N: Fn([f64; 2]) -> f64> Generator2D for Custom<2, N> {}
impl<N: Fn([f64; 3]) -> f64> Generator3D for Custom<3, N> {}
impl<N: Fn([f64; 4]) -> f64> Generator4D for Custom<4, N> {}

impl<const D: usize, N: Fn([f64; D]) -> f64> Custom<D, N> {
    #[inline]
    pub fn new(noise: N) -> Self {
        Self { noise }
    }
}

impl<const D: usize, N: Fn([f64; D]) -> f64> Generator<D> for Custom<D, N> {
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        (self.noise)(point)
    }
}
