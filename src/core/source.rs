use crate::core::sources::{
    Checkerboard, Constant, Custom, ImprovedPerlin, Perlin, Simplex, Value, Worley,
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

    pub fn worley(seed: u64) -> Worley<D> {
        Worley::new(seed)
    }

    pub fn checkerboard() -> Checkerboard<D> {
        Checkerboard::new()
    }

    pub fn custom<F: Fn([f64; D]) -> f64>(f: F) -> Custom<D, F> {
        Custom::new(f)
    }
}
