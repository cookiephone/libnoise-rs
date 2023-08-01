use crate::generator::Generator;

use super::math::tensor_indices;
use std::ops::{Index, IndexMut};

pub struct NoiseBuffer<const D: usize> {
    pub shape: [usize; D],
    pub(crate) offsets: [usize; D],
    pub buffer: Vec<f64>,
}

impl<const D: usize> Index<&[usize]> for NoiseBuffer<D> {
    type Output = f64;
    fn index(&self, index: &[usize]) -> &Self::Output {
        let idx = self.flat_index(index);
        &self.buffer[idx]
    }
}

impl<const D: usize> IndexMut<&[usize]> for NoiseBuffer<D> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.buffer[idx]
    }
}

impl<const D: usize> NoiseBuffer<D> {
    fn new_empty(shape: [usize; D]) -> Self {
        let bufsize = shape.iter().product();
        Self {
            shape,
            offsets: precompute_flat_index_offsets(&shape).try_into().unwrap(),
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

impl NoiseBuffer<1> {
    pub fn new<G: Generator<1>>(shape: [usize; 1], generator: G) -> Self {
        let mut noisebuf = Self::new_empty(shape);
        for point in tensor_indices(&shape) {
            noisebuf[&point] = generator.sample([point[0] as f64]);
        }
        noisebuf
    }
}

impl NoiseBuffer<2> {
    pub fn new<G: Generator<2>>(shape: [usize; 2], generator: G) -> Self {
        let mut noisebuf = Self::new_empty(shape);
        for point in tensor_indices(&shape) {
            noisebuf[&point] = generator.sample([point[0] as f64, point[1] as f64]);
        }
        noisebuf
    }
}

impl NoiseBuffer<3> {
    pub fn new<G: Generator<3>>(shape: [usize; 3], generator: G) -> Self {
        let mut noisebuf = Self::new_empty(shape);
        for point in tensor_indices(&shape) {
            noisebuf[&point] =
                generator.sample([point[0] as f64, point[1] as f64, point[2] as f64]);
        }
        noisebuf
    }
}

impl NoiseBuffer<4> {
    pub fn new<G: Generator<4>>(shape: [usize; 4], generator: G) -> Self {
        let mut noisebuf = Self::new_empty(shape);
        for point in tensor_indices(&shape) {
            noisebuf[&point] = generator.sample([
                point[0] as f64,
                point[1] as f64,
                point[2] as f64,
                point[3] as f64,
            ]);
        }
        noisebuf
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
