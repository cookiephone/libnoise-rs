mod utils;
use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use utils::*;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];
const SHAPE_4D: &[usize] = &[30, 30, 30, 30];

fn bench_opensimplex2_noise2d(c: &mut Criterion) {
    opensimplex2::fast::noise2(SEED as i64, 0.0, 0.0); // warmup init-once things
    c.bench_function("opensimplex2_fast_noise2d", |b| {
        b.iter(|| noise_2d_bencher(opensimplex2::fast::noise2, SEED as i64, SHAPE_2D, SCALE));
    });
}

fn bench_opensimplex2_noise3d(c: &mut Criterion) {
    opensimplex2::fast::noise3_Fallback(SEED as i64, 0.0, 0.0, 0.0); // warmup init-once things
    c.bench_function("opensimplex2_fast_noise3d", |b| {
        b.iter(|| {
            noise_3d_bencher(
                opensimplex2::fast::noise3_Fallback,
                SEED as i64,
                SHAPE_3D,
                SCALE,
            )
        });
    });
}

fn bench_opensimplex2_noise4d(c: &mut Criterion) {
    opensimplex2::fast::noise4_Fallback(SEED as i64, 0.0, 0.0, 0.0, 0.0); // warmup init-once things
    c.bench_function("opensimplex2_fast_noise4d", |b| {
        b.iter(|| {
            noise_4d_bencher(
                opensimplex2::fast::noise4_Fallback,
                SEED as i64,
                SHAPE_4D,
                SCALE,
            )
        });
    });
}

fn bench(c: &mut Criterion) {
    bench_opensimplex2_noise2d(c);
    bench_opensimplex2_noise3d(c);
    bench_opensimplex2_noise4d(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}

criterion_main!(benches);
