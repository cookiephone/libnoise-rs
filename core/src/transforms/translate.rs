pub fn apply<F, const D: usize>(generator: F, translation: [f64; D]) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
{
    move |seed, mut point| {
        point.iter_mut().zip(translation).for_each(|(x, dx)| *x += dx);
        generator(seed, point).abs()
    }
}
