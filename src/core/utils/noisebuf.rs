use crate::core::generator::Generator;
use itertools::Itertools;
use std::ops::{Index, IndexMut};

/// A struct for generating an n-dimensional array and efficiently filling it with noise values.
///
/// This struct represents a simple n-dimensional array which is stored as a flat vector. When
/// creating a new [`NoiseBuffer`] using the [`new()`] method, only the length along each
/// dimension and a noise generator, that is, an object implementing [`Generator`], must be
/// provided. The n-dimensional array is then filled with noise values. The implementation
/// ensures, that the noise is computed in a way such that the underlying flat vector is written
/// sequentially for maximal cache performance.
///
/// As [`NoiseBuffer`] implements the [`Index`] and [`IndexMut`] traits, it is possible to index
/// the n-dimensional array with an index array of the appropriate length.
///
/// # Creating a noise buffer
///
/// A noise buffer can be easily and efficiently created using the [`new()`] method. The resulting
/// buffer will be filled with noise values according to the provided noise generator.
///
/// When filling the buffer, the specific value at a given buffer index is computed by sampling
/// the generator with the index interpreted as coordinates in n-dimensional space. This means
/// that the buffer samples the generator on points of a hypergrid:
///
/// ```
/// # use libnoise::{Source, Generator, NoiseBuffer};
/// // create a generator
/// let generator = Source::simplex(42);
///
/// // create a new noise buffer filled with noise values for each point
/// let buf = NoiseBuffer::<3>::new([30, 20, 25], &generator);
///
/// assert_eq!(buf[[17, 9, 21]], generator.sample([17.0, 9.0, 21.0]));
/// ```
///
/// The scale or position of the
/// grid can be modified by calling adapters such as [`scale()`], [`translate()`], or [`rotate()`]
/// on the generator before using it to create a [`NoiseBuffer`].
///
/// [`new()`]: NoiseBuffer::new
/// [`scale()`]: Generator::scale
/// [`translate()`]: Generator::translate
/// [`rotate()`]: crate::Generator2D::rotate
#[derive(Clone, Debug)]
pub struct NoiseBuffer<const D: usize> {
    /// Stores the length of the n-dimensional array along each dimension.
    pub shape: [usize; D],
    /// Stores offsets which are used to convert n-dimensional coordinates to flat vector indices.
    pub offsets: [usize; D],
    /// The underlying flat vector storing the noise values.
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
            /// Creates a new noise buffer with the given `shape` and filled with noise generated
            /// by the given `generator`. For further detail see the
            /// [Creating a noise buffer](#creating-a-noise-buffer) section.
            pub fn new<G: Generator<$dim>>(shape: [usize; $dim], generator: &G) -> Self {
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
