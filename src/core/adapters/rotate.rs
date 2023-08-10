use crate::core::generator::{Generator, Generator2D, Generator3D, Generator4D};

/// A generator which rotates input points before passing them to the underlying generator.
///
/// For details, see the documentation of [`rotate()`]. Typically, this struct is not meant
/// to be used directly. Instead, [`rotate()`] implemented by [`Rotate`], should be used
/// to create [`Rotate`].
///
/// [`rotate()`]: Generator2D::rotate
#[derive(Clone, Copy, Debug)]
pub struct Rotate<const D: usize, const P: usize, G>
where
    G: Generator<D>,
{
    generator: G,
    rotation: [f64; P],
}

impl<G: Generator<2>> Generator2D for Rotate<2, 1, G> {}
impl<G: Generator<3>> Generator3D for Rotate<3, 3, G> {}
impl<G: Generator<4>> Generator4D for Rotate<4, 6, G> {}

impl<const D: usize, const P: usize, G> Rotate<D, P, G>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, rotation: [f64; P]) -> Self {
        Self {
            generator,
            rotation,
        }
    }
}

impl<G: Generator<2>> Generator<2> for Rotate<2, 1, G> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        let x = point[0];
        let y = point[1];

        let sin_theta = self.rotation[0].sin();
        let cos_theta = self.rotation[0].cos();

        let xr = x * cos_theta - y * sin_theta;
        let yr = x * sin_theta + y * cos_theta;

        self.generator.sample([xr, yr])
    }
}

impl<G: Generator<3>> Generator<3> for Rotate<3, 3, G> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        let x = point[0];
        let y = point[1];
        let z = point[2];

        let sin_a = self.rotation[0].sin();
        let cos_a = self.rotation[0].cos();

        let sin_b = self.rotation[1].sin();
        let cos_b = self.rotation[1].cos();

        let sin_g = self.rotation[2].sin();
        let cos_g = self.rotation[2].cos();
        
        let xr = cos_b*(x*cos_g + y*sin_g) + z*(-sin_b);
        let yr = sin_a*(sin_b*(x*cos_g + y*sin_g) + z*cos_b) - cos_a*(x*sin_g - y*cos_g);
        let zr = cos_a*(sin_b*(x*cos_g + y*sin_g) + z*cos_b) + sin_a*(x*sin_g - y*cos_g);

        self.generator.sample([xr, yr, zr])
    }
}

impl<G: Generator<4>> Generator<4> for Rotate<4, 6, G> {
    fn sample(&self, point: [f64; 4]) -> f64 {
        let x = point[0];
        let y = point[1];
        let z = point[2];
        let w = point[3];
        let sin_alpha = self.rotation[0].sin();
        let cos_alpha = self.rotation[0].cos();
        let sin_beta = self.rotation[1].sin();
        let cos_beta = self.rotation[1].cos();
        let sin_gamma = self.rotation[2].sin();
        let cos_gamma = self.rotation[2].cos();
        let sin_delta = self.rotation[3].sin();
        let cos_delta = self.rotation[3].cos();
        let sin_epsilon = self.rotation[4].sin();
        let cos_epsilon = self.rotation[4].cos();
        let sin_digamma = self.rotation[5].sin();
        let cos_digamma = self.rotation[5].cos();
        // i am too lazy to simplify this, i cannot verify correctness easily either
        let x1 = cos_alpha * cos_beta * cos_gamma;
        let y1 = -cos_epsilon * sin_alpha * cos_delta
            - cos_epsilon * cos_alpha * sin_beta * sin_delta
            - sin_epsilon * cos_alpha * cos_beta * sin_gamma;
        let z1 = -sin_alpha * (-sin_epsilon * cos_delta * sin_digamma - sin_delta * cos_digamma)
            - cos_alpha
                * sin_beta
                * (cos_delta * cos_digamma - sin_epsilon * sin_delta * sin_digamma)
            - cos_epsilon * cos_alpha * cos_beta * sin_gamma * sin_digamma;
        let w1 = -sin_alpha * (sin_delta * sin_digamma - sin_epsilon * cos_delta * cos_digamma)
            - cos_alpha
                * sin_beta
                * (-sin_epsilon * sin_delta * cos_digamma - cos_delta * sin_digamma)
            - cos_epsilon * cos_alpha * cos_beta * sin_gamma * cos_digamma;
        let x2 = sin_alpha * cos_beta * cos_gamma;
        let y2 = -cos_epsilon * sin_alpha * sin_beta * sin_delta
            - sin_epsilon * sin_alpha * cos_beta * sin_gamma
            + cos_epsilon * cos_alpha * cos_delta;
        let z2 = -sin_alpha
            * sin_beta
            * (cos_delta * cos_digamma - sin_epsilon * sin_delta * sin_digamma)
            - cos_epsilon * sin_alpha * cos_beta * sin_gamma * sin_digamma
            + cos_alpha * (-sin_epsilon * cos_delta * sin_digamma - sin_delta * cos_digamma);
        let w2 = -sin_alpha
            * sin_beta
            * (-sin_epsilon * sin_delta * cos_digamma - cos_delta * sin_digamma)
            - sin_alpha * cos_beta * sin_gamma * cos_digamma * cos_epsilon
            + cos_alpha * (sin_delta * sin_digamma - sin_epsilon * cos_delta * cos_digamma);
        let x3 = sin_beta * cos_gamma;
        let y3 = cos_beta * sin_gamma * cos_epsilon - sin_epsilon * sin_beta * sin_gamma;
        let z3 = cos_beta * (cos_delta * cos_digamma - sin_epsilon * sin_delta * sin_digamma)
            - sin_beta * sin_gamma * sin_digamma * sin_epsilon;
        let w3 = cos_beta * (-sin_epsilon * sin_delta * cos_digamma - cos_delta * sin_digamma)
            - sin_beta * sin_gamma * cos_digamma * cos_epsilon;
        let x4 = sin_gamma;
        let y4 = sin_epsilon * cos_gamma;
        let z4 = cos_gamma * sin_digamma * cos_epsilon;
        let w4 = cos_gamma * cos_digamma * cos_epsilon;
        let xr = x * x1 + y * y1 + z * z1 + w * w1;
        let yr = x * x2 + y * y2 + z * z2 + w * w2;
        let zr = x * x3 + y * y3 + z * z3 + w * w3;
        let wr = x * x4 + y * y4 + z * z4 + w * w4;
        self.generator.sample([xr, yr, zr, wr])
    }
}
