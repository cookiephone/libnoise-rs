use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, PlottingBackend};
use itertools::Itertools;
use noise::*;

fn bench_noise2d(noise_generator: &Simplex, x_max: usize, y_max: usize, scale: f64) {
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(noise_generator.noise2d(black_box(x), black_box(y)));
    }
}

fn bench(c: &mut Criterion) {
    let noise_generator = Simplex::new(0, 256);
    c.bench_function("noise2d", |b| {
        b.iter(|| bench_noise2d(&noise_generator, 3000, 3000, 0.05));
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
