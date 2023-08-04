use super::{Simplex, Value};

pub struct Source<const D: usize> {}

impl<const D: usize> Source<D> {
    pub fn simplex(seed: u64) -> Simplex<D> {
        Simplex::new(seed)
    }

    pub fn value(seed: u64) -> Value<D> {
        Value::new(seed)
    }
}
