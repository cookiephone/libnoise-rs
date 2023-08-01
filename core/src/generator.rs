pub trait Generator<const D: usize> {
    fn sample(&self, point: [f64; D]) -> f64;
}
