use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// A generator blending the underlying generator with a given other generator based on the
/// value supplied by a control-generator.
///
/// For details, see the documentation of [`blend()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`blend()`] implemented by [`Generator`], should be used
/// to create [`Blend`].
///
/// [`blend()`]: Generator::blend
#[derive(Clone, Copy, Debug)]
pub struct Blend<const D: usize, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    generator_a: GA,
    generator_b: GB,
    generator_control: GC,
}

impl<GA: Generator<1>, GB: Generator<1>, GC: Generator<1>> Generator1D for Blend<1, GA, GB, GC> {}
impl<GA: Generator<2>, GB: Generator<2>, GC: Generator<2>> Generator2D for Blend<2, GA, GB, GC> {}
impl<GA: Generator<3>, GB: Generator<3>, GC: Generator<3>> Generator3D for Blend<3, GA, GB, GC> {}
impl<GA: Generator<4>, GB: Generator<4>, GC: Generator<4>> Generator4D for Blend<4, GA, GB, GC> {}

impl<const D: usize, GA, GB, GC> Blend<D, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    #[inline]
    pub fn new(generator_a: GA, generator_b: GB, generator_control: GC) -> Self {
        Self {
            generator_a,
            generator_b,
            generator_control,
        }
    }
}

impl<const D: usize, GA, GB, GC> Generator<D> for Blend<D, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        let a = self.generator_a.sample(point);
        let b = self.generator_b.sample(point);
        let t = self.generator_control.sample(point) * 0.5 + 0.5;
        a + t * (b - a)
    }
}
