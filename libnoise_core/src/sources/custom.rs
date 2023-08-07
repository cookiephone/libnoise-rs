use crate::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

macro_rules! impl_custom_noise {
    ($dim:literal, $name:ident, $generatorname:ident) => {
        #[derive(Clone)]
        pub struct $name<N> {
            noise: N,
        }

        impl<N: Fn([f64; $dim]) -> f64> $generatorname for $name<N> {}

        impl<N: Fn([f64; $dim]) -> f64> $name<N> {
            #[inline]
            pub fn new(noise: N) -> Self {
                Self { noise }
            }
        }

        impl<N: Fn([f64; $dim]) -> f64> Generator<$dim> for $name<N> {
            #[inline]
            fn sample(&self, point: [f64; $dim]) -> f64 {
                (self.noise)(point)
            }
        }
    };
}

impl_custom_noise!(1, Custom1D, Generator1D);
impl_custom_noise!(2, Custom2D, Generator2D);
impl_custom_noise!(3, Custom3D, Generator3D);
impl_custom_noise!(4, Custom4D, Generator4D);
