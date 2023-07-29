use criterion::black_box;

use crate::inputgen::cartesian_lattice_points;

pub(crate) fn noise_1d_bencher<F, R, S>(noise: F, seed: S, shape: &[usize], scale: f64)
where
    F: Fn(S, f64) -> R,
    R: num::Float,
    S: Copy,
{
    for point in cartesian_lattice_points(shape, scale) {
        black_box(noise(
            seed,
            black_box(point[0] * scale),
        ));
    }
}

pub(crate) fn noise_2d_bencher<F, R, S>(noise: F, seed: S, shape: &[usize], scale: f64)
where
    F: Fn(S, f64, f64) -> R,
    R: num::Float,
    S: Copy,
{
    for point in cartesian_lattice_points(shape, scale) {
        black_box(noise(
            seed,
            black_box(point[0] * scale),
            black_box(point[1] * scale),
        ));
    }
}
