pub fn apply<F, const D: usize>(generator: F, min: f64, max: f64) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
{
    move |seed, point| generator(seed, point).clamp(min, max)
}
