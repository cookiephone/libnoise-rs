use super::inputgen::cartesian_lattice_points;
use criterion::black_box;

pub(crate) fn noise_bencher<F, R, S, const D: usize>(noise: F, seed: S, shape: &[usize], scale: f64)
where
    F: Fn(S, [f64; D]) -> R,
    R: num::Float,
    S: Copy,
{
    for mut point in cartesian_lattice_points(shape, scale) {
        point.iter_mut().for_each(|x| *x *= scale);
        black_box(noise(seed, black_box(point.try_into().unwrap())));
    }
}
