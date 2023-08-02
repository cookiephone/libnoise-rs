use crate::generator::Generator;

pub struct Fractal<const D: usize, G: Generator<D>> {
    generator: G,
    octaves: u32,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    persistence: f64,
    normalization_factor: f64,
}

impl<const D: usize, G: Generator<D>> Fractal<D, G> {
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

impl<G: Generator<1>> Generator<1> for Fractal<1, G> {
    fn sample(&self, point: [f64; 1]) -> f64 {
        let mut noise = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;
        for _ in 0..self.octaves {
            noise += amp * self.generator.sample([point[0] * freq]);
            freq *= self.lacunarity;
            amp *= self.persistence;
        }
        noise * self.normalization_factor
    }
}

impl<G: Generator<2>> Generator<2> for Fractal<2, G> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        let mut noise = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;
        for _ in 0..self.octaves {
            noise += amp * self.generator.sample([point[0] * freq, point[1] * freq]);
            freq *= self.lacunarity;
            amp *= self.persistence;
        }
        noise * self.normalization_factor
    }
}

impl<G: Generator<3>> Generator<3> for Fractal<3, G> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        let mut noise = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;
        for _ in 0..self.octaves {
            noise += amp
                * self
                    .generator
                    .sample([point[0] * freq, point[1] * freq, point[2] * freq]);
            freq *= self.lacunarity;
            amp *= self.persistence;
        }
        noise * self.normalization_factor
    }
}

impl<G: Generator<4>> Generator<4> for Fractal<4, G> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        let mut noise = 0.0;
        let mut amp = self.amplitude;
        let mut freq = self.frequency;
        for _ in 0..self.octaves {
            noise += amp
                * self.generator.sample([
                    point[0] * freq,
                    point[1] * freq,
                    point[2] * freq,
                    point[3] * freq,
                ]);
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
