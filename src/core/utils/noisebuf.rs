use crate::core::generator::Generator;
use itertools::Itertools;
use std::ops::{Index, IndexMut};

pub struct NoiseBuffer<const D: usize> {
    pub shape: [usize; D],
    pub(crate) offsets: [usize; D],
    pub buffer: Vec<f64>,
}

macro_rules! impl_indexing {
    ($dim:literal) => {
        impl Index<[usize; $dim]> for NoiseBuffer<$dim> {
            type Output = f64;
            fn index(&self, index: [usize; $dim]) -> &Self::Output {
                let idx = self.flat_index(index);
                &self.buffer[idx]
            }
        }
        
        impl IndexMut<[usize; $dim]> for NoiseBuffer<$dim> {
            fn index_mut(&mut self, index: [usize; $dim]) -> &mut Self::Output {
                let idx = self.flat_index(index);
                &mut self.buffer[idx]
            }
        }
    };
}

macro_rules! impl_new {
    ($dim:literal) => {
        impl NoiseBuffer<$dim> {
            pub fn new<G: Generator<$dim>>(shape: [usize; $dim], generator: G) -> Self {
                let mut noisebuf = Self::new_empty(shape);
                for point in noisebuf.tensor_indices() {
                    noisebuf[point] = generator.sample(point.map(|x| x as f64));
                }
                noisebuf
            }
        }
    };
}

impl_indexing!(1);
impl_indexing!(2);
impl_indexing!(3);
impl_indexing!(4);

impl_new!(1);
impl_new!(2);
impl_new!(3);
impl_new!(4);

impl<const D: usize> NoiseBuffer<D> {
    fn new_empty(shape: [usize; D]) -> Self {
        let bufsize = shape.iter().product();
        Self {
            shape,
            offsets: precompute_flat_index_offsets(&shape).try_into().unwrap(),
            buffer: vec![0.0; bufsize],
        }
    }

    fn flat_index(&self, index: [usize; D]) -> usize {
        index
            .iter()
            .zip(&self.offsets)
            .map(|(idx, offset)| idx * offset)
            .sum()
    }

    pub(crate) fn tensor_indices(&self) -> impl Iterator<Item = [usize; D]> {
        self.shape
            .iter()
            .map(|&dim_size| 0..dim_size)
            .multi_cartesian_product()
            .map(|point| point.try_into().unwrap())
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
