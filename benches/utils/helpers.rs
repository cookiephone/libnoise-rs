use libnoise::Generator;
use super::inputgen::cartesian_lattice_points;
use criterion::black_box;

pub(crate) fn noise_bencher<const D: usize, G: Generator<D>>(generator: &G, shape: &[usize], scale: f64) {
    for mut point in cartesian_lattice_points(shape, scale) {
        point.iter_mut().for_each(|x| *x *= scale);
        black_box(generator.sample(black_box(point.try_into().unwrap())));
    }
}
