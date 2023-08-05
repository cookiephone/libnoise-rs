use crate::sources::{
    Constant, Custom1D, Custom2D, Custom3D, Custom4D, ImprovedPerlin, Perlin, Simplex, Value,
};

pub struct Source<const D: usize> {}

impl<const D: usize> Source<D> {
    pub fn constant(value: f64) -> Constant<D> {
        Constant::new(value)
    }

    pub fn simplex(seed: u64) -> Simplex<D> {
        Simplex::new(seed)
    }

    pub fn value(seed: u64) -> Value<D> {
        Value::new(seed)
    }

    pub fn perlin(seed: u64) -> Perlin<D> {
        Perlin::new(seed)
    }

    pub fn improved_perlin(seed: u64) -> ImprovedPerlin<D> {
        ImprovedPerlin::new(seed)
    }
}

impl Source<1> {
    pub fn custom1d<N: Fn([f64; 1]) -> f64>(noise: N) -> Custom1D<N> {
        Custom1D::new(noise)
    }
}

impl Source<2> {
    pub fn custom2d<N: Fn([f64; 2]) -> f64>(noise: N) -> Custom2D<N> {
        Custom2D::new(noise)
    }
}

impl Source<3> {
    pub fn custom3d<N: Fn([f64; 3]) -> f64>(noise: N) -> Custom3D<N> {
        Custom3D::new(noise)
    }
}

impl Source<4> {
    pub fn custom4d<N: Fn([f64; 4]) -> f64>(noise: N) -> Custom4D<N> {
        Custom4D::new(noise)
    }
}
