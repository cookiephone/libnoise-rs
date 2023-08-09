use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// Create a generator applying an `fbm()`-like effect on the underlying generator.
///
/// For details, see the documentation of [`ridgedmulti()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`ridgedmulti()`] implemented by [`Generator`], should be used
/// to create [`RidgedMulti`].
///
/// [`fbm()`]: Generator:fbm
/// [`ridgedmulti()`]: Generator::ridgedmulti
#[derive(Clone, Copy, Debug)]
pub struct RidgedMulti<const D: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    octaves: u32,
    frequency: f64,
    lacunarity: f64,
    attenuation: f64,
    normalization_factor: f64,
}

impl<G: Generator<1>> Generator1D for RidgedMulti<1, G> {}
impl<G: Generator<2>> Generator2D for RidgedMulti<2, G> {}
impl<G: Generator<3>> Generator3D for RidgedMulti<3, G> {}
impl<G: Generator<4>> Generator4D for RidgedMulti<4, G> {}

impl<const D: usize, G> RidgedMulti<D, G>
where
    G: Generator<D>,
{
    #[inline]
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
        impl<G: Generator<$dim>> Generator<$dim> for RidgedMulti<$dim, G> {
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

#[inline]
fn compute_normalization_factor(octaves: u32, attenuation: f64) -> f64 {
    1.0 / (0..octaves).fold(0.0, |acc, octave| {
        acc + (1.0 / attenuation).powi(octave as i32)
    })
}
