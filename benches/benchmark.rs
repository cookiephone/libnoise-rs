use criterion::{black_box, criterion_group, criterion_main, Criterion, PlottingBackend};
use itertools::Itertools;

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

fn bench_opensimplex2_noise2d(x_max: usize, y_max: usize, scale: f64) {
    for (x, y) in (0..x_max)
        .cartesian_product(0..y_max)
        .map(|(x, y)| (x as f64 * scale, y as f64 * scale))
    {
        black_box(opensimplex2::fast::noise2(
            black_box(42),
            black_box(x),
            black_box(y),
        ));
    }
}

fn bench(c: &mut Criterion) {
    noise::simplex::noise2d(42, 0.0, 0.0); // warmup init-once things
    c.bench_function("simplex_noise2d", |b| {
        b.iter(|| bench_simplex_noise2d(42, 2000, 2000, 0.033));
    });
}

fn bench_competitors(c: &mut Criterion) {
    opensimplex2::fast::noise2(42, 0.0, 0.0);  // warmup init-once things
    c.bench_function("opensimplex2_noise2d", |b| {
        b.iter(|| bench_opensimplex2_noise2d(2000, 2000, 0.033));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench, bench_competitors
}
criterion_main!(benches);
