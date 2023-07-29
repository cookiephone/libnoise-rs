use crate::helpers::{noise_2d_bencher, noise_3d_bencher};
use criterion::Criterion;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];

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

pub(crate) fn bench_competitors(c: &mut Criterion) {
    bench_opensimplex2_noise2d(c);
    bench_opensimplex2_noise3d(c);
}
