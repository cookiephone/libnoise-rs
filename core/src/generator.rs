use crate::adapters;
use std::marker::Sized;

pub trait Generator<const D: usize> {
    fn sample(&self, point: [f64; D]) -> f64;

    fn fbm(
        self,
        octaves: u32,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        persistencee: f64,
    ) -> adapters::Fbm<D, Self>
    where
        Self: Sized,
    {
        adapters::Fbm::new(
            self,
            octaves,
            frequency,
            amplitude,
            lacunarity,
            persistencee,
        )
    }

    fn scale(self, scale: [f64; D]) -> adapters::Scale<D, Self>
    where
        Self: Sized,
    {
        adapters::Scale::new(self, scale)
    }

    fn translate(self, translation: [f64; D]) -> adapters::Translate<D, Self>
    where
        Self: Sized,
    {
        adapters::Translate::new(self, translation)
    }

    fn rotate<const P: usize>(self, rotation: [f64; P]) -> adapters::Rotate<P, Self>
    where
        Self: Sized,
    {
        adapters::Rotate::new(self, rotation)
    }

    fn abs(self) -> adapters::Abs<Self>
    where
        Self: Sized,
    {
        adapters::Abs::new(self)
    }

    fn add(self, offset: f64) -> adapters::Add<Self>
    where
        Self: Sized,
    {
        adapters::Add::new(self, offset)
    }

    fn clamp(self, min: f64, max: f64) -> adapters::Clamp<Self>
    where
        Self: Sized,
    {
        adapters::Clamp::new(self, min, max)
    }

    fn lambda<L: Fn(f64) -> f64>(self, lambda: L) -> adapters::Lambda<Self, L>
    where
        Self: Sized,
    {
        adapters::Lambda::new(self, lambda)
    }

    fn mul(self, scale: f64) -> adapters::Mul<Self>
    where
        Self: Sized,
    {
        adapters::Mul::new(self, scale)
    }

    fn neg(self) -> adapters::Neg<Self>
    where
        Self: Sized,
    {
        adapters::Neg::new(self)
    }
}
