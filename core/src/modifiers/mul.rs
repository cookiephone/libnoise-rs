pub fn apply<F, const D: usize, G>(generator: F, scale_factor: f64) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
    G: Fn(f64) -> f64,
{
    move |seed, point| generator(seed, point) * scale_factor
}