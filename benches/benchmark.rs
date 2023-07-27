use criterion::{black_box, criterion_group, criterion_main, Criterion, PlottingBackend};
use itertools::Itertools;
use std::time::Duration;

fn bench_simplex_noise2d(seed: u64, x_max: usize, y_max: usize, scale: f64) {
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(noise::simplex::noise2d(
            black_box(seed),
            black_box(x),
            black_box(y),
        ));
    }
}

fn bench_opensimplex_noise2d(x_max: usize, y_max: usize, scale: f64) {
    let noise_generator = opensimplex_noise_rs::OpenSimplexNoise::new(Some(0));
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(noise_generator.eval_2d(black_box(x), black_box(y)));
    }
}

fn bench_opensimplex2_noise2d(x_max: usize, y_max: usize, scale: f64) {
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(opensimplex2::fast::noise2(
            black_box(0),
            black_box(x),
            black_box(y),
        ));
    }
}

fn bench(c: &mut Criterion) {
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| bench_simplex_noise2d(42, 3000, 3000, 0.033));
    });
}

fn bench_competitors(c: &mut Criterion) {
    c.bench_function("opensimplex_noise2d", |b| {
        b.iter(|| bench_opensimplex_noise2d(3000, 3000, 0.033));
    });
    c.bench_function("opensimplex2_noise2d", |b| {
        b.iter(|| bench_opensimplex2_noise2d(3000, 3000, 0.033));
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
