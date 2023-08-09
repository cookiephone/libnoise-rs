use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use libnoise::prelude::*;

const SCALE: f64 = 0.033;
const SHAPE_1D: &[usize] = &[1000000];
const SHAPE_2D: &[usize] = &[1000, 1000];
const SHAPE_3D: &[usize] = &[100, 100, 100];
const SHAPE_4D: &[usize] = &[30, 30, 30, 30];

macro_rules! impl_generator {
    () => {
        Source::simplex(42).ridgedmulti(3, 0.013, 2.0, 2.0)
    };
}

fn bench(c: &mut Criterion) {
    devtools::benchtools::bench_noise1d(c, "ridgedmulti_1d", SHAPE_1D, SCALE, &impl_generator!());
    devtools::benchtools::bench_noise2d(c, "ridgedmulti_2d", SHAPE_2D, SCALE, &impl_generator!());
    devtools::benchtools::bench_noise3d(c, "ridgedmulti_3d", SHAPE_3D, SCALE, &impl_generator!());
    devtools::benchtools::bench_noise4d(c, "ridgedmulti_4d", SHAPE_4D, SCALE, &impl_generator!());
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}

criterion_main!(benches);
