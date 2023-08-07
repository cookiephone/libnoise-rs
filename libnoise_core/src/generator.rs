use crate::adapters;
use std::marker::Sized;

pub trait Generator<const D: usize>: Sized {
    fn sample(&self, point: [f64; D]) -> f64;

    #[inline]
    fn fbm(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Fbm<Self> {
        adapters::Fbm::new(self, octaves, frequency, lacunarity, persistence)
    }

    #[inline]
    fn billow(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Billow<Self> {
        adapters::Billow::new(self, octaves, frequency, lacunarity, persistence)
    }

    #[inline]
    fn ridgedmulti(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        attenuation: f64,
    ) -> adapters::RidgedMulti<Self> {
        adapters::RidgedMulti::new(self, octaves, frequency, lacunarity, attenuation)
    }

    #[inline]
    fn scale(self, scale: [f64; D]) -> adapters::Scale<D, Self> {
        adapters::Scale::new(self, scale)
    }

    #[inline]
    fn translate(self, translation: [f64; D]) -> adapters::Translate<D, Self> {
        adapters::Translate::new(self, translation)
    }

    #[inline]
    fn abs(self) -> adapters::Abs<Self> {
        adapters::Abs::new(self)
    }

    #[inline]
    fn exp(self) -> adapters::Exp<Self> {
        adapters::Exp::new(self)
    }

    #[inline]
    fn add(self, offset: f64) -> adapters::Add<Self> {
        adapters::Add::new(self, offset)
    }

    #[inline]
    fn powi(self, exponent: i32) -> adapters::Pow<Self, i32> {
        adapters::Pow::new(self, exponent)
    }

    #[inline]
    fn powf(self, exponent: f64) -> adapters::Pow<Self, f64> {
        adapters::Pow::new(self, exponent)
    }

    #[inline]
    fn power<G>(self, other: G) -> adapters::Power<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Power::new(self, other)
    }

    #[inline]
    fn product<G>(self, other: G) -> adapters::Product<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Product::new(self, other)
    }

    #[inline]
    fn sum<G>(self, other: G) -> adapters::Sum<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Sum::new(self, other)
    }

    #[inline]
    fn clamp(self, min: f64, max: f64) -> adapters::Clamp<Self> {
        adapters::Clamp::new(self, min, max)
    }

    #[inline]
    fn lambda<L>(self, lambda: L) -> adapters::Lambda<Self, L>
    where
        L: Fn(f64) -> f64,
    {
        adapters::Lambda::new(self, lambda)
    }

    #[inline]
    fn mul(self, scale: f64) -> adapters::Mul<Self> {
        adapters::Mul::new(self, scale)
    }

    #[inline]
    fn neg(self) -> adapters::Neg<Self> {
        adapters::Neg::new(self)
    }

    #[inline]
    fn blend<G, GC>(self, other: G, control: GC) -> adapters::Blend<Self, G, GC>
    where
        G: Generator<D>,
        GC: Generator<D>,
    {
        adapters::Blend::new(self, other, control)
    }

    #[inline]
    fn select<G, GC>(
        self,
        other: G,
        control: GC,
        selection_min: f64,
        selection_max: f64,
    ) -> adapters::Select<Self, G, GC>
    where
        G: Generator<D>,
        GC: Generator<D>,
    {
        adapters::Select::new(self, other, control, selection_min, selection_max)
    }

    #[inline]
    fn min<G>(self, other: G) -> adapters::Min<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Min::new(self, other)
    }

    #[inline]
    fn max<G>(self, other: G) -> adapters::Max<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Max::new(self, other)
    }
}

pub trait Generator1D: Generator<1> {
    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<1>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator2D: Generator<2> {
    #[inline]
    fn rotate(self, rotation: [f64; 1]) -> adapters::Rotate<1, Self> {
        adapters::Rotate::new(self, rotation)
    }

    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator3D: Generator<3> {
    #[inline]
    fn rotate(self, rotation: [f64; 3]) -> adapters::Rotate<3, Self> {
        adapters::Rotate::new(self, rotation)
    }

    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

pub trait Generator4D: Generator<4> {
    #[inline]
    fn rotate(self, rotation: [f64; 6]) -> adapters::Rotate<6, Self> {
        adapters::Rotate::new(self, rotation)
    }

    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    #[inline]
    fn displace_w<GA>(self, displacement_generator: GA) -> adapters::Displace<3, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}
