use crate::generator::Generator;

#[derive(Clone)]
pub struct Fbm<const D: usize, G> {
    generator: G,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
    normalization_factor: f64,
}

impl<const D: usize, G> Fbm<D, G> {
    pub fn new(
        generator: G,
        octaves: u32,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> Self {
        let normalization_factor = compute_normalization_factor(octaves, amplitude, persistence);
        Self {
            generator,
            octaves,
            frequency,
            amplitude,
            lacunarity,
            persistence,
            normalization_factor,
        }
    }
}

impl<const D: usize, G: Generator<D>> Generator<D> for Fbm<D, G> {
    fn sample(&self, point: [f64; D]) -> f64 {
        let mut noise = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;
        for _ in 0..self.octaves {
            noise += amp * self.generator.sample(point.map(|x| x * freq));
            freq *= self.lacunarity;
            amp *= self.persistence;
        }
        noise * self.normalization_factor
    }
}

fn compute_normalization_factor(octaves: u32, amplitude: f64, persistence: f64) -> f64 {
    1.0 / (0..octaves).fold(0.0, |acc, octave| {
        acc + amplitude * persistence.powi(octave as i32)
    })
}
