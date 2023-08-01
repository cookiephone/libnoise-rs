mod utils;
use libnoise::sources;
use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use utils::*;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];
const SHAPE_4D: &[usize] = &[30, 30, 30, 30];

fn bench_simplex_noise2d(c: &mut Criterion) {
    let generator = sources::Simplex::new(SEED);
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| noise_bencher::<2,_>(&generator, SHAPE_2D, SCALE));
    });
}

fn bench_simplex_noise3d(c: &mut Criterion) {
    let generator = sources::Simplex::new(SEED);
    c.bench_function("simplex_noise3d", |b| {
        b.iter(|| noise_bencher::<3,_>(&generator, SHAPE_3D, SCALE));
    });
}

fn bench_simplex_noise4d(c: &mut Criterion) {
    let generator = sources::Simplex::new(SEED);
    c.bench_function("simplex_noise4d", |b| {
        b.iter(|| noise_bencher::<4,_>(&generator, SHAPE_4D, SCALE));
    });
}

fn bench(c: &mut Criterion) {
    bench_simplex_noise2d(c);
    bench_simplex_noise3d(c);
    bench_simplex_noise4d(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}

criterion_main!(benches);
