use crate::inputgen::cartesian_lattice_points;
use criterion::{black_box, Criterion};

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[2000, 2000];

fn noise_2d_bencher<F, R, S>(noise: F, seed: S, shape: &[usize], scale: f64)
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

fn bench_simplex_noise2d(c: &mut Criterion) {
    noise::simplex::noise2d(SEED, 0.0, 0.0); // warmup init-once things
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| noise_2d_bencher(noise::simplex::noise2d, SEED, SHAPE_2D, SCALE));
    });
}

fn bench_opensimplex2_noise2d(c: &mut Criterion) {
    opensimplex2::fast::noise2(SEED as i64, 0.0, 0.0); // warmup init-once things
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| noise_2d_bencher(opensimplex2::fast::noise2, SEED as i64, SHAPE_2D, SCALE));
    });
}

pub(crate) fn bench(c: &mut Criterion) {
    bench_simplex_noise2d(c);
}

pub(crate) fn bench_competitors(c: &mut Criterion) {
    bench_opensimplex2_noise2d(c);
}
