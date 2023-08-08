use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Constant<const D: usize> {
    value: f64,
}

impl Generator1D for Constant<1> {}
impl Generator2D for Constant<2> {}
impl Generator3D for Constant<3> {}
impl Generator4D for Constant<4> {}

impl<const D: usize> Constant<D> {
    #[inline]
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl<const D: usize> Generator<D> for Constant<D> {
    #[inline]
    fn sample(&self, _point: [f64; D]) -> f64 {
        self.value
    }
}
