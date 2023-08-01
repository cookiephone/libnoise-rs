pub fn apply<F, const D: usize, G>(generator: F, lambda: G) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
    G: Fn(f64) -> f64,
{
    move |seed, point| lambda(generator(seed, point))
}
