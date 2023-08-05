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
    ) -> adapters::Fbm<Self>
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

    fn lambda<L>(self, lambda: L) -> adapters::Lambda<Self, L>
    where
        L: Fn(f64) -> f64,
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

pub trait Generator1D: Generator<1> {
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<1>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator2D: Generator<2> {
    fn rotate(self, rotation: [f64; 1]) -> adapters::Rotate<1, Self>
    where
        Self: Sized,
    {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<2>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<2>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator3D: Generator<3> {
    fn rotate(self, rotation: [f64; 3]) -> adapters::Rotate<3, Self>
    where
        Self: Sized,
    {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<3>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<3>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<3>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator4D: Generator<4> {
    fn rotate(self, rotation: [f64; 6]) -> adapters::Rotate<6, Self>
    where
        Self: Sized,
    {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<4>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<4>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<4>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_w<GA>(self, displacement_generator: GA) -> adapters::Displace<3, Self, GA>
    where
        GA: Generator<4>,
        Self: Sized,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}
