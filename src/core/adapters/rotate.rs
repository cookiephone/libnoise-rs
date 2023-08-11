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

        let xr = cos_b * (x * cos_g + y * sin_g) + z * (-sin_b);
        let yr =
            sin_a * (sin_b * (x * cos_g + y * sin_g) + z * cos_b) - cos_a * (x * sin_g - y * cos_g);
        let zr =
            cos_a * (sin_b * (x * cos_g + y * sin_g) + z * cos_b) + sin_a * (x * sin_g - y * cos_g);

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

        let xr = cos_alpha
            * (x * cos_beta * cos_gamma
                + sin_beta
                    * (sin_delta
                        * (sin_epsilon * (z * sin_digamma + w * cos_digamma) - y * cos_epsilon)
                        + cos_delta * (-z * cos_digamma + w * sin_digamma))
                + sin_gamma
                    * (cos_beta
                        * (cos_epsilon * (-z * sin_digamma - w * cos_digamma) - y * sin_epsilon)))
            + sin_alpha
                * (cos_delta
                    * (sin_epsilon * (z * sin_digamma + w * cos_digamma) - y * cos_epsilon)
                    + sin_delta * (z * cos_digamma - w * sin_digamma));

        let yr = sin_alpha
            * (cos_beta
                * (sin_gamma
                    * (cos_epsilon * (-z * sin_digamma - w * cos_digamma) - y * sin_epsilon)
                    + x * cos_gamma)
                + sin_beta
                    * (sin_delta
                        * (sin_epsilon * (z * sin_digamma + w * cos_digamma) - y * cos_epsilon)
                        + cos_delta * (-z * cos_digamma + w * sin_digamma)))
            + cos_alpha
                * (cos_delta
                    * (sin_epsilon * (-z * sin_digamma - w * cos_digamma) + y * cos_epsilon)
                    + sin_delta * (-z * cos_digamma + w * sin_digamma));

        let zr = cos_beta
            * (sin_epsilon * (sin_delta * (-z * sin_digamma - w * cos_digamma))
                + cos_delta * (z * cos_digamma - w * sin_digamma)
                + y * sin_gamma * cos_epsilon)
            + sin_beta
                * (sin_gamma
                    * (sin_epsilon * (-y - z * sin_digamma) - w * cos_digamma * cos_epsilon)
                    + x * cos_gamma);

        let wr = cos_gamma * (cos_epsilon * (z * sin_digamma + w * cos_digamma) + y * sin_epsilon)
            + x * sin_gamma;

        self.generator.sample([xr, yr, zr, wr])
    }
}
