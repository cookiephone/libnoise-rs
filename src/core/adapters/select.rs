use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// Create a generator selecting the result of either the underlying generator or that of a given
/// other generator based on whether the value supplied by a control-generator lies within the
/// provided interval.
///
/// For details, see the documentation of [`select()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`select()`] implemented by [`Generator`], should be used
/// to create [`Select`].
///
/// [`select()`]: Generator::select
#[derive(Clone, Copy, Debug)]
pub struct Select<const D: usize, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    generator_a: GA,
    generator_b: GB,
    generator_control: GC,
    selection_min: f64,
    selection_max: f64,
}

impl<GA: Generator<1>, GB: Generator<1>, GC: Generator<1>> Generator1D for Select<1, GA, GB, GC> {}
impl<GA: Generator<2>, GB: Generator<2>, GC: Generator<2>> Generator2D for Select<2, GA, GB, GC> {}
impl<GA: Generator<3>, GB: Generator<3>, GC: Generator<3>> Generator3D for Select<3, GA, GB, GC> {}
impl<GA: Generator<4>, GB: Generator<4>, GC: Generator<4>> Generator4D for Select<4, GA, GB, GC> {}

impl<const D: usize, GA, GB, GC> Select<D, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    #[inline]
    pub fn new(
        generator_a: GA,
        generator_b: GB,
        generator_control: GC,
        selection_min: f64,
        selection_max: f64,
    ) -> Self {
        Self {
            generator_a,
            generator_b,
            generator_control,
            selection_min,
            selection_max,
        }
    }
}

impl<const D: usize, GA, GB, GC> Generator<D> for Select<D, GA, GB, GC>
where
    GA: Generator<D>,
    GB: Generator<D>,
    GC: Generator<D>,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        match self.generator_control.sample(point) {
            t if self.selection_min <= t && t <= self.selection_max => {
                self.generator_a.sample(point)
            }
            _ => self.generator_b.sample(point),
        }
    }
}
