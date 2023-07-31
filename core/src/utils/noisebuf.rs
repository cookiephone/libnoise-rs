use super::math::tensor_indices;
use std::ops::{Index, IndexMut};

pub struct NoiseBuffer {
    pub shape: Vec<usize>,
    pub seed: u64,
    pub(crate) offsets: Vec<usize>,
    pub buffer: Vec<f64>,
}

impl Index<&[usize]> for NoiseBuffer {
    type Output = f64;
    fn index(&self, index: &[usize]) -> &Self::Output {
        let idx = self.flat_index(index);
        &self.buffer[idx]
    }
}

impl IndexMut<&[usize]> for NoiseBuffer {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.buffer[idx]
    }
}

impl NoiseBuffer {
    pub fn new1d<F>(shape: &[usize], generator: F, seed: u64) -> Self
    where
        F: Fn(u64, f64) -> f64,
    {
        let mut noisebuf = Self::new_empty(shape, seed);
        for p in tensor_indices(shape) {
            noisebuf[&p] = generator(seed, p[0] as f64);
        }
        noisebuf
    }

    pub fn new2d<F>(shape: &[usize], generator: F, seed: u64) -> Self
    where
        F: Fn(u64, f64, f64) -> f64,
    {
        let mut noisebuf = Self::new_empty(shape, seed);
        for p in tensor_indices(shape) {
            noisebuf[&p] = generator(seed, p[0] as f64, p[1] as f64);
        }
        noisebuf
    }

    pub fn new3d<F>(shape: &[usize], generator: F, seed: u64) -> Self
    where
        F: Fn(u64, f64, f64, f64) -> f64,
    {
        let mut noisebuf = Self::new_empty(shape, seed);
        for p in tensor_indices(shape) {
            noisebuf[&p] = generator(seed, p[0] as f64, p[1] as f64, p[2] as f64);
        }
        noisebuf
    }

    pub fn new4d<F>(shape: &[usize], generator: F, seed: u64) -> Self
    where
        F: Fn(u64, f64, f64, f64, f64) -> f64,
    {
        let mut noisebuf = Self::new_empty(shape, seed);
        for p in tensor_indices(shape) {
            noisebuf[&p] = generator(seed, p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64);
        }
        noisebuf
    }

    pub(crate) fn new_empty(shape: &[usize], seed: u64) -> Self {
        let bufsize = shape.iter().product();
        Self {
            shape: shape.to_vec(),
            seed,
            offsets: precompute_flat_index_offsets(shape),
            buffer: vec![0.0; bufsize],
        }
    }

    fn flat_index(&self, index: &[usize]) -> usize {
        index
            .iter()
            .zip(&self.offsets)
            .map(|(idx, offset)| idx * offset)
            .sum()
    }
}

pub(crate) fn precompute_flat_index_offsets(shape: &[usize]) -> Vec<usize> {
    let offsets = shape
        .iter()
        .rev()
        .scan(1, |state, dim_size| {
            let offset = Some(*state);
            *state *= dim_size;
            offset
        })
        .collect::<Vec<usize>>();
    offsets.into_iter().rev().collect()
}
