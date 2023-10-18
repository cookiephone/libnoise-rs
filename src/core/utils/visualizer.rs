use crate::core::generator::Generator;
use crate::core::utils::noisebuf::NoiseBuffer;
use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType, GrayImage, ImageError,
};
use itertools::Itertools;
use std::{
    fs::OpenOptions,
    io::Error,
    ops::{Index, IndexMut},
};

/// A struct for visualizing the output of a generator.
///
/// This struct represents a simple way to quickly visualize the output of a [`Generator`] by
/// building a [`NoiseBuffer`] of a given size, populating it with data, and creating an PNG or
/// GIF file visualizing said data.
///
/// In the 1D case, the visualization is a grayscale band of one pixel in height and with the
/// provided length. In the 2D case, the visualization is an image of the provided dimensions.
/// In the 3D case, the visualization is an image providing an isometric view on a cube
/// representing the buffer. In the 4D case, the visualization is equivalent to the 3D case,
/// except the result is an animation with the 4th dimension mapping to time.
///
/// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
/// <strong>Note:</strong>
/// This struct is meant to be used to get an idea of what a generator is doing. Especially the
/// 1D, 3D, and 4D cases are not suited for usage besides debugging, as the main goal of this
/// library is to provide an efficient and modular way to creating a noise generation pipeline.
/// </p>
///
/// The usage of this struct is simple and analogous to that of [`NoiseBuffer`]:
///
/// ```
/// # use libnoise::{Source, NoiseBuffer, Visualizer};
/// # use tempdir::TempDir;
/// // create a generator
/// let generator = Source::simplex(42);
///
/// // create a visualizer and use it to visualize the output of the generator
/// let path = "output.png";
/// # let tmp_dir = TempDir::new("libnoise").unwrap();
/// # let path = &tmp_dir.path().join(path).into_os_string().into_string().unwrap();
/// Visualizer::<3>::new([30, 20, 25], &generator).write_to_file(path).unwrap();
/// ```
///
/// In fact, a visualizer can be created from a [`NoiseBuffer`] by simply converting it
/// to a [`Visualizer`]:
///
/// ```
/// # use libnoise::{Source, NoiseBuffer, Visualizer};
/// # use tempdir::TempDir;
/// // create a generator
/// let generator = Source::simplex(42);
///
/// // create a noise buffer
/// let buf = NoiseBuffer::<3>::new([30, 20, 25], &generator);
///
/// // create a visualizer and use it to visualize the output of the generator
/// let path = "output.png";
/// # let tmp_dir = TempDir::new("libnoise").unwrap();
/// # let path = &tmp_dir.path().join(path).into_os_string().into_string().unwrap();
/// Visualizer::from(buf).write_to_file(path);
/// ```
#[derive(Clone, Debug)]
pub struct Visualizer<const D: usize> {
    /// Stores the length of the underlying n-dimensional array along each dimension.
    shape: [usize; D],
    /// Stores offsets which are used to convert n-dimensional coordinates to flat vector indices.
    offsets: [usize; D],
    /// The underlying flat vector storing the noise values as `u8` integers.
    pixel_buffer: Vec<u8>,
}

impl<const D: usize> Index<&[usize]> for Visualizer<D> {
    type Output = u8;
    fn index(&self, index: &[usize]) -> &Self::Output {
        let idx = self.flat_index(index);
        &self.pixel_buffer[idx]
    }
}

impl<const D: usize> IndexMut<&[usize]> for Visualizer<D> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.pixel_buffer[idx]
    }
}

impl<const D: usize> From<NoiseBuffer<D>> for Visualizer<D> {
    fn from(noisebuf: NoiseBuffer<D>) -> Self {
        Self {
            shape: noisebuf.shape,
            offsets: noisebuf.offsets,
            pixel_buffer: noisebuf.buffer.into_iter().map(norm_to_u8).collect(),
        }
    }
}

impl<const D: usize> Visualizer<D> {
    fn flat_index(&self, index: &[usize]) -> usize {
        index
            .iter()
            .zip(&self.offsets)
            .map(|(idx, offset)| idx * offset)
            .sum()
    }
}

impl Visualizer<1> {
    /// Creates a new [`Visualizer`] with the given `shape` and filled with noise generated
    /// by the given `generator`. For further detail see the
    /// [struct-level documentation](Visualizer).
    pub fn new<G: Generator<1>>(shape: [usize; 1], generator: &G) -> Self {
        NoiseBuffer::<1>::new(shape, generator).into()
    }

    /// Write a PNG file to the given `path`, visualizing the output of the provided
    /// generator. For further detail see the [struct-level documentation](Visualizer).
    pub fn write_to_file(&self, path: &str) -> Result<(), ImageError> {
        let image =
            GrayImage::from_raw(self.shape[0] as u32, 1, self.pixel_buffer.clone()).unwrap();
        image.save(path)?;
        Ok(())
    }
}

impl Visualizer<2> {
    /// Creates a new [`Visualizer`] with the given `shape` and filled with noise generated
    /// by the given `generator`. For further detail see the
    /// [struct-level documentation](Visualizer).
    pub fn new<G: Generator<2>>(shape: [usize; 2], generator: &G) -> Self {
        NoiseBuffer::<2>::new(shape, generator).into()
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), ImageError> {
        let image = GrayImage::from_raw(
            self.shape[1] as u32,
            self.shape[0] as u32,
            self.pixel_buffer.clone(),
        )
        .unwrap();
        image.save(path)?;
        Ok(())
    }
}

impl Visualizer<3> {
    /// Creates a new [`Visualizer`] with the given `shape` and filled with noise generated
    /// by the given `generator`. For further detail see the
    /// [struct-level documentation](Visualizer).
    pub fn new<G: Generator<3>>(shape: [usize; 3], generator: &G) -> Self {
        NoiseBuffer::<3>::new(shape, generator).into()
    }

    /// Write a PNG file to the given `path`, visualizing the output of the provided
    /// generator. For further detail see the [struct-level documentation](Visualizer).
    pub fn write_to_file(&self, path: &str) -> Result<(), ImageError> {
        let scale = 0.45;
        let center = (self.shape[0] as f64 * 0.5, self.shape[1] as f64 * 0.5);
        let mut buf = vec![0; self.shape[0] * self.shape[1]];
        for z_idx in (0..self.shape[2]).rev() {
            for p in tensor_indices(&[self.shape[0], self.shape[1]]) {
                if let Some(buf_idx) =
                    xyz_screen_to_buff_indices(p[0], p[1], z_idx, center.0, center.1, scale)
                {
                    buf[p[0] * self.shape[1] + p[1]] = self[&[buf_idx.0, buf_idx.1, buf_idx.2]];
                }
            }
        }

        let image = GrayImage::from_raw(self.shape[1] as u32, self.shape[0] as u32, buf).unwrap();
        image.save(path)?;
        Ok(())
    }
}

impl Visualizer<4> {
    /// Creates a new [`Visualizer`] with the given `shape` and filled with noise generated
    /// by the given `generator`. For further detail see the
    /// [struct-level documentation](Visualizer).
    pub fn new<G: Generator<4>>(shape: [usize; 4], generator: &G) -> Self {
        NoiseBuffer::<4>::new(shape, generator).into()
    }

    /// Write a GIF file to the given `path`, visualizing the output of the provided
    /// generator. For further detail see the [struct-level documentation](Visualizer).
    pub fn write_to_file(&self, path: &str) -> Result<(), Error> {
        let file_out = OpenOptions::new().write(true).create(true).open(path)?;

        let mut encoder = GifEncoder::new(file_out);
        encoder.set_repeat(Repeat::Infinite).unwrap();

        let scale = 0.45;
        let center = (self.shape[0] as f64 * 0.5, self.shape[1] as f64 * 0.5);
        for t in 0..self.shape[3] {
            let mut buf = vec![0; self.shape[0] * self.shape[1]];
            for z_idx in (0..self.shape[2]).rev() {
                for p in tensor_indices(&[self.shape[0], self.shape[1]]) {
                    if let Some(buf_idx) =
                        xyz_screen_to_buff_indices(p[0], p[1], z_idx, center.0, center.1, scale)
                    {
                        buf[p[0] * self.shape[0] + p[1]] =
                            self[&[buf_idx.0, buf_idx.1, buf_idx.2, t]];
                    }
                }
            }

            buf = buf
                .into_iter()
                .flat_map(|val| std::iter::repeat(val).take(3))
                .collect();

            encoder
                .encode(
                    &buf,
                    self.shape[0] as u32,
                    self.shape[1] as u32,
                    ColorType::Rgb8,
                )
                .unwrap();
        }
        Ok(())
    }
}

pub(crate) fn norm_to_u8(x: f64) -> u8 {
    (127.5 + x * 127.5) as u8
}

fn xyz_screen_to_buff_indices(
    x: usize,
    y: usize,
    z: usize,
    center_x: f64,
    center_y: f64,
    scale: f64,
) -> Option<(usize, usize, usize)> {
    let mut x = x as f64;
    let mut y = y as f64;
    x -= center_x * (1.0 - scale) + scale * z as f64;
    y -= center_y;
    let xx = -(x + y / 3_f64.sqrt());
    let yy = 2.0 * y / 3_f64.sqrt() + xx;
    x = xx / scale + center_x;
    y = yy / scale + center_y;
    if x < 0.0 || y < 0.0 || x >= 2.0 * center_x || y >= 2.0 * center_y {
        None
    } else {
        Some((x as usize, y as usize, z))
    }
}

fn tensor_indices(shape: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    shape
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
}
