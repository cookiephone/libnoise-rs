use crate::generator::Generator;

#[derive(Clone)]
pub struct Displace<const A: usize, G, GA> {
    generator: G,
    displacement_generator: GA,
}

impl<const A: usize, G, GA> Displace<A, G, GA> {
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
    fn sample(&self, mut point: [f64; D]) -> f64 {
        point[A] += self.displacement_generator.sample(point);
        self.generator.sample(point)
    }
}
