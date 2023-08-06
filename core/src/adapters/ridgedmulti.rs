use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct RidgedMulti<G> {
    generator: G,
    octaves: u32,
    frequency: f64,
    lacunarity: f64,
    attenuation: f64,
    normalization_factor: f64,
}

impl<G: Generator<1>> Generator1D for RidgedMulti<G> {}
impl<G: Generator<2>> Generator2D for RidgedMulti<G> {}
impl<G: Generator<3>> Generator3D for RidgedMulti<G> {}
impl<G: Generator<4>> Generator4D for RidgedMulti<G> {}

impl<G> RidgedMulti<G> {
    pub fn new(
        generator: G,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        attenuation: f64,
    ) -> Self {
        let normalization_factor = compute_normalization_factor(octaves, attenuation);
        Self {
            generator,
            octaves,
            frequency,
            lacunarity,
            attenuation,
            normalization_factor,
        }
    }
}

macro_rules! impl_generator {
    ($dim:literal) => {
        impl<G: Generator<$dim>> Generator<$dim> for RidgedMulti<G> {
            fn sample(&self, point: [f64; $dim]) -> f64 {
                let mut noise = 0.0;
                let mut amp = 1.0;
                let mut freq = self.frequency;
                for _ in 0..self.octaves {
                    let mut layer = 1.0 - self.generator.sample(point.map(|x| x * freq)).abs();
                    layer *= layer;
                    layer *= amp;
                    noise += layer;
                    freq *= self.lacunarity;
                    amp = (layer / self.attenuation).clamp(0.0, 1.0);
                }
                (noise * self.normalization_factor).mul_add(2.0, -1.0)
            }
        }
    };
}

impl_generator!(1);
impl_generator!(2);
impl_generator!(3);
impl_generator!(4);

fn compute_normalization_factor(octaves: u32, attenuation: f64) -> f64 {
    1.0 / (0..octaves).fold(0.0, |acc, octave| {
        acc + (1.0 / attenuation).powi(octave as i32)
    })
}
