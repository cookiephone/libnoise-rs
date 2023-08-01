mod utils;
use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};
use libnoise::{combiners, sources, Generator};
use utils::{bench_noise1d, bench_noise2d, bench_noise3d, bench_noise4d, constants::*};

macro_rules! impl_generator {
    () => {
        combiners::Fractal::new(sources::Value::new(SEED), 3, 0.013, 1.0, 2.0, 0.5)
    };
}

fn generator1d() -> impl Generator<1> {
    impl_generator!()
}
fn generator2d() -> impl Generator<2> {
    impl_generator!()
}
fn generator3d() -> impl Generator<3> {
    impl_generator!()
}
fn generator4d() -> impl Generator<4> {
    impl_generator!()
}

fn bench(c: &mut Criterion) {
    bench_noise1d(c, "fractal_1d", SHAPE_1D, SCALE, &generator1d());
    bench_noise2d(c, "fractal_2d", SHAPE_2D, SCALE, &generator2d());
    bench_noise3d(c, "fractal_3d", SHAPE_3D, SCALE, &generator3d());
    bench_noise4d(c, "fractal_4d", SHAPE_4D, SCALE, &generator4d());
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}

criterion_main!(benches);
