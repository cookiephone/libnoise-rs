use super::NoiseBuffer;
use crate::generator::Generator;
use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType, GrayImage,
};
use itertools::Itertools;
use std::{
    fs::OpenOptions,
    ops::{Index, IndexMut},
};

pub struct Visualizer<const D: usize> {
    shape: [usize; D],
    offsets: [usize; D],
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
    pub fn new<G: Generator<1>>(shape: [usize; 1], generator: G) -> Self {
        NoiseBuffer::<1>::new(shape, generator).into()
    }

    pub fn write_to_file(&self, path: &str) {
        let image =
            GrayImage::from_raw(self.shape[0] as u32, 1, self.pixel_buffer.clone()).unwrap();
        image.save(path).unwrap();
    }
}

impl Visualizer<2> {
    pub fn new<G: Generator<2>>(shape: [usize; 2], generator: G) -> Self {
        NoiseBuffer::<2>::new(shape, generator).into()
    }

    pub fn write_to_file(&self, path: &str) {
        let image = GrayImage::from_raw(
            self.shape[1] as u32,
            self.shape[0] as u32,
            self.pixel_buffer.clone(),
        )
        .unwrap();
        image.save(path).unwrap();
    }
}

impl Visualizer<3> {
    pub fn new<G: Generator<3>>(shape: [usize; 3], generator: G) -> Self {
        NoiseBuffer::<3>::new(shape, generator).into()
    }

    pub fn write_to_file(&self, path: &str) {
        let scale = 0.45;
        let center = (self.shape[0] as f64 * 0.5, self.shape[1] as f64 * 0.5);
        let mut buf = vec![0; self.shape[0] * self.shape[1]];
        for z_idx in (0..self.shape[2]).rev() {
            for p in tensor_indices(&[self.shape[0], self.shape[1]]) {
                if let Some(buf_idx) =
                    xyz_screen_to_buff_indices(p[0], p[1], z_idx, center.0, center.1, scale)
                {
                    buf[p[0] * self.shape[0] + p[1]] = self[&[buf_idx.0, buf_idx.1, buf_idx.2]];
                }
            }
        }
        let image = GrayImage::from_raw(self.shape[1] as u32, self.shape[0] as u32, buf).unwrap();
        image.save(path).unwrap();
    }
}

impl Visualizer<4> {
    pub fn new<G: Generator<4>>(shape: [usize; 4], generator: G) -> Self {
        NoiseBuffer::<4>::new(shape, generator).into()
    }

    pub fn write_to_file(&self, path: &str) {
        let file_out = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
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
