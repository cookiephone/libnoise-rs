use std::sync::Once;

struct StaticNormalizationFactor {
    factor: Option<f64>,
    params: Option<(u32, f64, f64)>,
    sync: Once,
}

static mut NORMALIZATION_FACTOR: StaticNormalizationFactor = StaticNormalizationFactor {
    factor: None,
    params: None,
    sync: Once::new(),
};

fn compute_normalization_factor(octaves: u32, amplitude: f64, persistence: f64) -> f64 {
    1.0 / (0..octaves).fold(0.0, |acc, octave| {
        acc + amplitude * persistence.powi(octave as i32)
    })
}

fn get_normalization_factor(octaves: u32, amplitude: f64, persistence: f64) -> &'static f64 {
    unsafe {
        if NORMALIZATION_FACTOR
            .params
            .is_some_and(|old_params| old_params != (octaves, amplitude, persistence))
        {
            NORMALIZATION_FACTOR.sync = Once::new();
        }
        NORMALIZATION_FACTOR.sync.call_once(|| {
            NORMALIZATION_FACTOR.params = Some((octaves, amplitude, persistence));
            NORMALIZATION_FACTOR.factor = Some(compute_normalization_factor(
                octaves,
                amplitude,
                persistence,
            ));
        });
        NORMALIZATION_FACTOR.factor.as_ref().unwrap()
    }
}

pub fn apply<F, const D: usize>(
    generator: F,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
) -> impl Fn(u64, [f64; D]) -> f64
where
    F: Fn(u64, [f64; D]) -> f64,
{
    move |seed, point| {
        let mut noise = 0.0;
        let mut amp = amplitude;
        let mut freq = frequency;
        for _ in 0..octaves {
            noise += amp * generator(seed, point.map(|x| x * freq));
            freq *= lacunarity;
            amp *= persistence;
        }
        noise * get_normalization_factor(octaves, amplitude, persistence)
    }
}
