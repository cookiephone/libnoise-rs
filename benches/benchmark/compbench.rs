use crate::helpers::noise_2d_bencher;
use criterion::Criterion;

const SEED: u64 = 42;
const SCALE: f64 = 0.033;
const SHAPE_2D: &[usize] = &[2000, 2000];

fn bench_opensimplex2_noise2d(c: &mut Criterion) {
    opensimplex2::fast::noise2(SEED as i64, 0.0, 0.0); // warmup init-once things
    c.bench_function("opensimplex2_fast_noise2d", |b| {
        b.iter(|| noise_2d_bencher(opensimplex2::fast::noise2, SEED as i64, SHAPE_2D, SCALE));
    });
}

pub(crate) fn bench_competitors(c: &mut Criterion) {
    bench_opensimplex2_noise2d(c);
}
