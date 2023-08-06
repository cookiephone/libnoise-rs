use crate::adapters;
use std::marker::Sized;

pub trait Generator<const D: usize>: Sized {
    fn sample(&self, point: [f64; D]) -> f64;

    fn fbm(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Fbm<Self> {
        adapters::Fbm::new(self, octaves, frequency, lacunarity, persistence)
    }

    fn billow(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Billow<Self> {
        adapters::Billow::new(self, octaves, frequency, lacunarity, persistence)
    }

    fn ridgedmulti(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        attenuation: f64,
    ) -> adapters::RidgedMulti<Self> {
        adapters::RidgedMulti::new(self, octaves, frequency, lacunarity, attenuation)
    }

    fn scale(self, scale: [f64; D]) -> adapters::Scale<D, Self> {
        adapters::Scale::new(self, scale)
    }

    fn translate(self, translation: [f64; D]) -> adapters::Translate<D, Self> {
        adapters::Translate::new(self, translation)
    }

    fn abs(self) -> adapters::Abs<Self> {
        adapters::Abs::new(self)
    }

    fn exp(self) -> adapters::Exp<Self> {
        adapters::Exp::new(self)
    }

    fn add(self, offset: f64) -> adapters::Add<Self> {
        adapters::Add::new(self, offset)
    }

    fn powi(self, exponent: i32) -> adapters::Pow<Self, i32> {
        adapters::Pow::new(self, exponent)
    }

    fn powf(self, exponent: f64) -> adapters::Pow<Self, f64> {
        adapters::Pow::new(self, exponent)
    }

    fn clamp(self, min: f64, max: f64) -> adapters::Clamp<Self> {
        adapters::Clamp::new(self, min, max)
    }

    fn lambda<L>(self, lambda: L) -> adapters::Lambda<Self, L>
    where
        L: Fn(f64) -> f64,
    {
        adapters::Lambda::new(self, lambda)
    }

    fn mul(self, scale: f64) -> adapters::Mul<Self> {
        adapters::Mul::new(self, scale)
    }

    fn neg(self) -> adapters::Neg<Self> {
        adapters::Neg::new(self)
    }

    fn blend<G, GC>(self, other: G, control: GC) -> adapters::Blend<Self, G, GC>
    where
        G: Generator<D>,
        GC: Generator<D>,
    {
        adapters::Blend::new(self, other, control)
    }

    fn min<G>(self, other: G) -> adapters::Min<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Min::new(self, other)
    }

    fn max<G>(self, other: G) -> adapters::Max<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Max::new(self, other)
    }
}

pub trait Generator1D: Generator<1> {
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<1>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator2D: Generator<2> {
    fn rotate(self, rotation: [f64; 1]) -> adapters::Rotate<1, Self> {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator3D: Generator<3> {
    fn rotate(self, rotation: [f64; 3]) -> adapters::Rotate<3, Self> {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator4D: Generator<4> {
    fn rotate(self, rotation: [f64; 6]) -> adapters::Rotate<6, Self> {
        adapters::Rotate::new(self, rotation)
    }

    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    fn displace_w<GA>(self, displacement_generator: GA) -> adapters::Displace<3, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}
