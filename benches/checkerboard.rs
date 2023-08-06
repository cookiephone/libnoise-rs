mod utils;
use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use libnoise::prelude::*;
use utils::{bench_noise1d, bench_noise2d, bench_noise3d, bench_noise4d, constants::*};

macro_rules! impl_generator {
    () => {
        Source::checkerboard()
    };
}

fn bench(c: &mut Criterion) {
    bench_noise1d(c, "checkerboard_1d", SHAPE_1D, SCALE, &impl_generator!());
    bench_noise2d(c, "checkerboard_2d", SHAPE_2D, SCALE, &impl_generator!());
    bench_noise3d(c, "checkerboard_3d", SHAPE_3D, SCALE, &impl_generator!());
    bench_noise4d(c, "checkerboard_4d", SHAPE_4D, SCALE, &impl_generator!());
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}

criterion_main!(benches);
