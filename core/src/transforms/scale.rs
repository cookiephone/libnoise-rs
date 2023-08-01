pub fn apply<F, const D: usize>(generator: F, scale: [f64; D]) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
{
    move |seed, mut point| {
        point.iter_mut().zip(scale).for_each(|(x, sx)| *x *= sx);
        generator(seed, point)
    }
}
