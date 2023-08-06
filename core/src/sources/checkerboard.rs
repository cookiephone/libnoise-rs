use super::functional;
use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Checkerboard<const D: usize>;

impl Generator1D for Checkerboard<1> {}
impl Generator2D for Checkerboard<2> {}
impl Generator3D for Checkerboard<3> {}
impl Generator4D for Checkerboard<4> {}

#[allow(clippy::new_without_default)]
impl<const D: usize> Checkerboard<D> {
    pub fn new() -> Self {
        Self
    }
}

impl Generator<1> for Checkerboard<1> {
    fn sample(&self, point: [f64; 1]) -> f64 {
        functional::checkerboard::noise1d(point)
    }
}

impl Generator<2> for Checkerboard<2> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        functional::checkerboard::noise2d(point)
    }
}

impl Generator<3> for Checkerboard<3> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        functional::checkerboard::noise3d(point)
    }
}

impl Generator<4> for Checkerboard<4> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        functional::checkerboard::noise4d(point)
    }
}
