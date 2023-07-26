use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, PlottingBackend};
use itertools::Itertools;

fn bench_simplex_noise2d(x_max: usize, y_max: usize, scale: f64) {
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(noise::simplex::noise2d(black_box(x), black_box(y)));
    }
}

fn bench(c: &mut Criterion) {
    c.bench_function("noise2d", |b| {
        b.iter(|| bench_simplex_noise2d(3000, 3000, 0.05));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters)
        .measurement_time(Duration::from_secs(30));
    targets = bench
}
criterion_main!(benches);
