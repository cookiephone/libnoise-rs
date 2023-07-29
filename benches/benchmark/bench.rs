use crate::helpers::{noise_1d_bencher, noise_2d_bencher, noise_3d_bencher};
use criterion::Criterion;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_1D: &[usize] = &[8000];
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];

fn bench_simplex_noise1d(c: &mut Criterion) {
    noise::simplex::noise1d(SEED, 0.0); // warmup init-once things
    c.bench_function("simplex_noise1d", |b| {
        b.iter(|| noise_1d_bencher(noise::simplex::noise1d, SEED, SHAPE_1D, SCALE));
    });
}

fn bench_simplex_noise2d(c: &mut Criterion) {
    noise::simplex::noise2d(SEED, 0.0, 0.0); // warmup init-once things
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| noise_2d_bencher(noise::simplex::noise2d, SEED, SHAPE_2D, SCALE));
    });
}

fn bench_simplex_noise3d(c: &mut Criterion) {
    noise::simplex::noise2d(SEED, 0.0, 0.0); // warmup init-once things
    c.bench_function("simplex_noise3d", |b| {
        b.iter(|| noise_3d_bencher(noise::simplex::noise3d, SEED, SHAPE_3D, SCALE));
    });
}

pub(crate) fn bench(c: &mut Criterion) {
    //bench_simplex_noise1d(c);
    //bench_simplex_noise2d(c);
    bench_simplex_noise3d(c);
}
