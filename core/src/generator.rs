use crate::{combiners, modifiers, transforms};
use std::marker::Sized;

pub trait Generator<const D: usize> {
    fn sample(&self, point: [f64; D]) -> f64;

    fn fractal(
        self,
        octaves: u32,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        persistencee: f64,
    ) -> combiners::Fractal<D, Self>
    where
        Self: Sized,
    {
        combiners::Fractal::new(
            self,
            octaves,
            frequency,
            amplitude,
            lacunarity,
            persistencee,
        )
    }

    fn scale(self, scale: [f64; D]) -> transforms::Scale<D, Self>
    where
        Self: Sized,
    {
        transforms::Scale::new(self, scale)
    }

    fn translate(self, translation: [f64; D]) -> transforms::Translate<D, Self>
    where
        Self: Sized,
    {
        transforms::Translate::new(self, translation)
    }

    fn rotate<const P: usize>(self, rotation: [f64; P]) -> transforms::Rotate<D, P, Self>
    where
        Self: Sized,
    {
        transforms::Rotate::new(self, rotation)
    }

    fn abs(self) -> modifiers::Abs<D, Self>
    where
        Self: Sized,
    {
        modifiers::Abs::new(self)
    }

    fn add(self, offset: f64) -> modifiers::Add<D, Self>
    where
        Self: Sized,
    {
        modifiers::Add::new(self, offset)
    }

    fn clamp(self, min: f64, max: f64) -> modifiers::Clamp<D, Self>
    where
        Self: Sized,
    {
        modifiers::Clamp::new(self, min, max)
    }

    fn lambda<L: Fn(f64) -> f64>(self, lambda: L) -> modifiers::Lambda<D, Self, L>
    where
        Self: Sized,
    {
        modifiers::Lambda::new(self, lambda)
    }

    fn mul(self, scale: f64) -> modifiers::Mul<D, Self>
    where
        Self: Sized,
    {
        modifiers::Mul::new(self, scale)
    }

    fn neg(self) -> modifiers::Neg<D, Self>
    where
        Self: Sized,
    {
        modifiers::Neg::new(self)
    }
}
