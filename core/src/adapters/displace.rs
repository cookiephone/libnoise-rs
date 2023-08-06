use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

#[derive(Clone)]
pub struct Displace<const A: usize, G, GA> {
    generator: G,
    displacement_generator: GA,
}

impl<const A: usize, G: Generator<1>, GA: Generator<1>> Generator1D for Displace<A, G, GA> {}
impl<const A: usize, G: Generator<2>, GA: Generator<2>> Generator2D for Displace<A, G, GA> {}
impl<const A: usize, G: Generator<3>, GA: Generator<3>> Generator3D for Displace<A, G, GA> {}
impl<const A: usize, G: Generator<4>, GA: Generator<4>> Generator4D for Displace<A, G, GA> {}

impl<const A: usize, G, GA> Displace<A, G, GA> {
    #[inline]
    pub fn new(generator: G, displacement_generator: GA) -> Self {
        Self {
            generator,
            displacement_generator,
        }
    }
}

impl<const A: usize, const D: usize, G, GA> Generator<D> for Displace<A, G, GA>
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
