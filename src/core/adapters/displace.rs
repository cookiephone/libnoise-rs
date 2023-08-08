use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Displace<const D: usize, const A: usize, G, GA>
where
    G: Generator<D>,
    GA: Generator<D>,
{
    generator: G,
    displacement_generator: GA,
}

impl<const A: usize, G: Generator<1>, GA: Generator<1>> Generator1D for Displace<1, A, G, GA> {}
impl<const A: usize, G: Generator<2>, GA: Generator<2>> Generator2D for Displace<2, A, G, GA> {}
impl<const A: usize, G: Generator<3>, GA: Generator<3>> Generator3D for Displace<3, A, G, GA> {}
impl<const A: usize, G: Generator<4>, GA: Generator<4>> Generator4D for Displace<4, A, G, GA> {}

impl<const D: usize, const A: usize, G, GA> Displace<D, A, G, GA>
where
    G: Generator<D>,
    GA: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, displacement_generator: GA) -> Self {
        Self {
            generator,
            displacement_generator,
        }
    }
}

impl<const D: usize, const A: usize, G, GA> Generator<D> for Displace<D, A, G, GA>
where
    G: Generator<D>,
    GA: Generator<D>,
{
    #[inline]
    fn sample(&self, mut point: [f64; D]) -> f64 {
        point[A] += self.displacement_generator.sample(point);
        self.generator.sample(point)
    }
}
