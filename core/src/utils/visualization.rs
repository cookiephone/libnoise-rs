use std::{
    fs::OpenOptions,
    ops::{Index, IndexMut},
};

use super::{math::tensor_indices, NoiseBuffer};
use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType, GrayImage,
};

pub struct Visualizer {
    shape: Vec<usize>,
    offsets: Vec<usize>,
    pixel_buffer: Vec<u8>,
}

impl Index<&[usize]> for Visualizer {
    type Output = u8;
    fn index(&self, index: &[usize]) -> &Self::Output {
        let idx = self.flat_index(index);
        &self.pixel_buffer[idx]
    }
}

impl IndexMut<&[usize]> for Visualizer {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.pixel_buffer[idx]
    }
}

impl Visualizer {
    pub fn write_to_file(&self, path: &str) {
        match self.shape.len() {
            1 => self.write_to_file_1d(path),
            2 => self.write_to_file_2d(path),
            3 => self.write_to_file_3d(path),
            4 => unimplemented!("general 4d visualization is nontrivial"),
            _ => panic!("unsupported number of dimensions"),
        }
    }

    fn write_to_file_1d(&self, path: &str) {
        let image =
            GrayImage::from_raw(self.shape[0] as u32, 1, self.pixel_buffer.clone()).unwrap();
        image.save(path).unwrap();
    }

    fn write_to_file_2d(&self, path: &str) {
        let image = GrayImage::from_raw(
            self.shape[1] as u32,
            self.shape[0] as u32,
            self.pixel_buffer.clone(),
        )
        .unwrap();
        image.save(path).unwrap();
    }

    fn write_to_file_3d(&self, path: &str) {
        let file_out = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut encoder = GifEncoder::new(file_out);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for c in 0..self.shape[2] {
            let channel = tensor_indices(&[self.shape[0], self.shape[1]])
                .map(|p| self[&[p[0], p[1], c]])
                .flat_map(|val| std::iter::repeat(val).take(3))
                .collect::<Vec<u8>>();
            encoder
                .encode(
                    &channel,
                    self.shape[0] as u32,
                    self.shape[1] as u32,
                    ColorType::Rgb8,
                )
                .unwrap();
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

impl From<NoiseBuffer> for Visualizer {
    fn from(noisebuf: NoiseBuffer) -> Self {
        Self {
            shape: noisebuf.shape,
            offsets: noisebuf.offsets,
            pixel_buffer: noisebuf.buffer.into_iter().map(norm_to_u8).collect(),
        }
    }
}

pub(crate) fn norm_to_u8(x: f64) -> u8 {
    (127.5 + x * 127.5) as u8
}
