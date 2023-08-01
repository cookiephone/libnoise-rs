mod utils;
use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use utils::*;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];
const SHAPE_4D: &[usize] = &[30, 30, 30, 30];

fn bench_simplex_noise2d(c: &mut Criterion) {
    libnoise::generators::simplex::noise2d(SEED, [0.0, 0.0]); // warmup init-once things
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| {
            noise_bencher(
                libnoise::generators::simplex::noise2d,
                SEED,
                SHAPE_2D,
                SCALE,
            )
        });
    });
}

fn bench_simplex_noise3d(c: &mut Criterion) {
    libnoise::generators::simplex::noise3d(SEED, [0.0, 0.0, 0.0]); // warmup init-once things
    c.bench_function("simplex_noise3d", |b| {
        b.iter(|| {
            noise_bencher(
                libnoise::generators::simplex::noise3d,
                SEED,
                SHAPE_3D,
                SCALE,
            )
        });
    });
}

fn bench_simplex_noise4d(c: &mut Criterion) {
    libnoise::generators::simplex::noise4d(SEED, [0.0, 0.0, 0.0, 0.0]); // warmup init-once things
    c.bench_function("simplex_noise4d", |b| {
        b.iter(|| {
            noise_bencher(
                libnoise::generators::simplex::noise4d,
                SEED,
                SHAPE_4D,
                SCALE,
            )
        });
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
