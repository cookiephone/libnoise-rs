use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};

mod bench;
mod inputgen;

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench::bench, bench::bench_competitors
}
criterion_main!(benches);
