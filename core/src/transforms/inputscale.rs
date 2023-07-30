pub fn transform1d<F>(generator: F, scale_factor: f64) -> impl Fn(u64, f64) -> f64
where
    F: Fn(u64, f64) -> f64,
{
    move |seed, x| generator(seed, scale_factor * x)
}

pub fn transform2d<F>(generator: F, scale_factor: f64) -> impl Fn(u64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64) -> f64,
{
    move |seed, x, y| generator(seed, scale_factor * x, scale_factor * y)
}

pub fn transform3d<F>(generator: F, scale_factor: f64) -> impl Fn(u64, f64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64, f64) -> f64,
{
    move |seed, x, y, z| generator(seed, scale_factor * x, scale_factor * y, scale_factor * z)
}

pub fn transform4d<F>(generator: F, scale_factor: f64) -> impl Fn(u64, f64, f64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64, f64, f64) -> f64,
{
    move |seed, x, y, z, w| {
        generator(
            seed,
            scale_factor * x,
            scale_factor * y,
            scale_factor * z,
            scale_factor * w,
        )
    }
}
