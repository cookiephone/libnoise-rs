use crate::Generator;
use criterion::{black_box, Criterion};
use itertools::Itertools;

pub fn bench_noise1d<G: Generator<1>>(
    c: &mut Criterion,
    id: &str,
    shape: &[usize],
    scale: f64,
    generator: &G,
) {
    c.bench_function(id, |b| {
        b.iter(|| noise_bencher::<1, _>(generator, shape, scale));
    });
}

pub fn bench_noise2d<G: Generator<2>>(
    c: &mut Criterion,
    id: &str,
    shape: &[usize],
    scale: f64,
    generator: &G,
) {
    c.bench_function(id, |b| {
        b.iter(|| noise_bencher::<2, _>(generator, shape, scale));
    });
}

pub fn bench_noise3d<G: Generator<3>>(
    c: &mut Criterion,
    id: &str,
    shape: &[usize],
    scale: f64,
    generator: &G,
) {
    c.bench_function(id, |b| {
        b.iter(|| noise_bencher::<3, _>(generator, shape, scale));
    });
}

pub fn bench_noise4d<G: Generator<4>>(
    c: &mut Criterion,
    id: &str,
    shape: &[usize],
    scale: f64,
    generator: &G,
) {
    c.bench_function(id, |b| {
        b.iter(|| noise_bencher::<4, _>(generator, shape, scale));
    });
}

fn noise_bencher<const D: usize, G: Generator<D>>(generator: &G, shape: &[usize], scale: f64) {
    for point in cartesian_lattice_points(shape, scale) {
        black_box(generator.sample(black_box(point.try_into().unwrap())));
    }
}

fn tensor_indices(shape: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    shape
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
}

fn cartesian_lattice_points(shape: &[usize], scale: f64) -> impl Iterator<Item = Vec<f64>> {
    tensor_indices(shape).map(move |point| {
        point
            .iter()
            .map(|&component| component as f64 * scale)
            .collect()
    })
}
