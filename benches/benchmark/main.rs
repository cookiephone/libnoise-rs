mod bench;
mod compbench;
mod helpers;
mod inputgen;

use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench::bench, compbench::bench_competitors
}

criterion_main!(benches);
