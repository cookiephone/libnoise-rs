pub fn apply<F, const D: usize>(generator: F, scale_factor: f64) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
{
    move |seed, mut point| {
        point.iter_mut().for_each(|x| *x *= scale_factor);
        generator(seed, point)
    }
}
