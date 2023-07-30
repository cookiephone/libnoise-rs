pub fn transform1d<F>(
    generator: F,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
) -> impl Fn(u64, f64) -> f64
where
    F: Fn(u64, f64) -> f64,
{
    move |seed, x| {
        let mut noise = 0.0;
        let mut amp = amplitude;
        let mut freq = frequency;
        for _ in 0..octaves {
            noise += amp * generator(seed, freq * x);
            freq *= lacunarity;
            amp *= persistence;
        }
        noise
    }
}

pub fn transform2d<F>(
    generator: F,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
) -> impl Fn(u64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64) -> f64,
{
    move |seed, x, y| {
        let mut noise = 0.0;
        let mut amp = amplitude;
        let mut freq = frequency;
        for _ in 0..octaves {
            noise += amp * generator(seed, freq * x, freq * y);
            freq *= lacunarity;
            amp *= persistence;
        }
        noise
    }
}

pub fn transform3d<F>(
    generator: F,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
) -> impl Fn(u64, f64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64, f64) -> f64,
{
    move |seed, x, y, z| {
        let mut noise = 0.0;
        let mut amp = amplitude;
        let mut freq = frequency;
        for _ in 0..octaves {
            noise += amp * generator(seed, freq * x, freq * y, freq * z);
            freq *= lacunarity;
            amp *= persistence;
        }
        noise
    }
}

pub fn transform4d<F>(
    generator: F,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
) -> impl Fn(u64, f64, f64, f64, f64) -> f64
where
    F: Fn(u64, f64, f64, f64, f64) -> f64,
{
    move |seed, x, y, z, w| {
        let mut noise = 0.0;
        let mut amp = amplitude;
        let mut freq = frequency;
        for _ in 0..octaves {
            noise += amp * generator(seed, freq * x, freq * y, freq * z, freq * w);
            freq *= lacunarity;
            amp *= persistence;
        }
        noise
    }
}
