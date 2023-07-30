use image::{
    codecs::gif::{GifEncoder, Repeat},
    ColorType, GrayImage,
};
use itertools::Itertools;
use std::fs::OpenOptions;

#[test]
fn test_simplex_noise1d() {
    let noise = noise::simplex::noise1d;
    test_noise1d(noise, "simplex_1d.png");
}

#[test]
fn test_simplex_noise2d() {
    let noise = noise::simplex::noise2d;
    test_noise2d(noise, "simplex_2d.png");
}

#[test]
fn test_simplex_noise3d() {
    let noise = noise::simplex::noise3d;
    test_noise3d(noise, "simplex_3d.gif");
}

#[test]
fn test_simplex_noise4d() {
    let noise = noise::simplex::noise4d;
    test_noise4d(noise, "simplex_4d.gif");
}

#[test]
fn test_fractal_simplex_noise1d() {
    let noise =
        noise::transforms::fractal::transform1d(noise::simplex::noise1d, 4, 1.0, 1.0, 2.0, 0.5);
    test_noise1d(noise, "fractal_simplex_1d.png");
}

#[test]
fn test_fractal_simplex_noise2d() {
    let noise =
        noise::transforms::fractal::transform2d(noise::simplex::noise2d, 4, 1.0, 1.0, 2.0, 0.5);
    test_noise2d(noise, "fractal_simplex_2d.png");
}

#[test]
fn test_fractal_simplex_noise3d() {
    let noise =
        noise::transforms::fractal::transform3d(noise::simplex::noise3d, 4, 1.0, 1.0, 2.0, 0.5);
    test_noise3d(noise, "fractal_simplex_3d.gif");
}

#[test]
fn test_fractal_simplex_noise4d() {
    let noise =
        noise::transforms::fractal::transform4d(noise::simplex::noise4d, 4, 1.0, 1.0, 2.0, 0.5);
    test_noise4d(noise, "fractal_simplex_4d.gif");
}

fn test_noise1d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64) -> f64,
{
    let (w, h) = (3000, 300);
    let mut data = vec![vec![255; w]; h];
    for j in 0..w {
        let noise = generator(42, j as f64 * 0.013);
        let noise = (h as f64 * (noise * 0.5 + 0.5)) as usize;
        for (i, row) in data.iter_mut().enumerate() {
            if noise.abs_diff(i) < (h / 30) {
                row[j] = 0;
            }
        }
    }
    let data = data.into_iter().flatten().collect::<Vec<u8>>();
    let image = GrayImage::from_raw(w as u32, h as u32, data).unwrap();
    image.save(path).unwrap();
}

fn test_noise2d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64) -> f64,
{
    let (w, h) = (3000, 3000);
    let mut data = vec![vec![0.0; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        data[i][j] = generator(42, i as f64 * 0.013, j as f64 * 0.013);
    }
    let data = data.into_iter().flatten().collect::<Vec<f64>>();
    let data = data
        .iter()
        .map(|&val| (127.5 + val * 127.5) as u8)
        .collect::<Vec<u8>>();
    let image = GrayImage::from_raw(w as u32, h as u32, data).unwrap();
    image.save(path).unwrap();
}

fn test_noise3d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64, f64) -> f64,
{
    let (w, h, d) = (300, 300, 50);
    let mut data = vec![vec![vec![0; w]; h]; d];

    for (i, j, k) in [d, h, w]
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
        .map(|p| (p[0], p[1], p[2]))
    {
        let noise = generator(42, i as f64 * 0.033, j as f64 * 0.033, k as f64 * 0.033);
        data[i][j][k] = (noise * 172.5 + 172.5) as u8
    }
    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for channel in data.iter().map(|c| {
        c.iter()
            .flatten()
            .flat_map(|&val| std::iter::repeat(val).take(3))
            .collect::<Vec<u8>>()
    }) {
        encoder
            .encode(&channel, w as u32, h as u32, ColorType::Rgb8)
            .unwrap();
    }
}

fn test_noise4d<F>(generator: F, path: &str)
where
    F: Fn(u64, f64, f64, f64, f64) -> f64,
{
    let size = 50;
    let mut slice_yzw = vec![vec![vec![0; size]; size]; size];
    let mut slice_xzw = vec![vec![vec![0; size]; size]; size];
    let mut slice_xyw = vec![vec![vec![0; size]; size]; size];
    let mut slice_xyz = vec![vec![vec![0; size]; size]; size];
    for (i, j, k, l) in [size, size, size, size]
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
        .map(|p| (p[0], p[1], p[2], p[3]))
    {
        let ifs = i as f64 * 0.033;
        let jfs = j as f64 * 0.033;
        let kfs = k as f64 * 0.033;
        let lfs = l as f64 * 0.033;
        let noise_yzw = generator(42, 0.0, jfs, kfs, lfs);
        let noise_xzw = generator(42, ifs, 0.0, kfs, lfs);
        let noise_xyw = generator(42, ifs, jfs, 0.0, lfs);
        let noise_xyz = generator(42, ifs, jfs, kfs, 0.0);
        slice_yzw[j][k][l] = (noise_yzw * 172.5 + 172.5) as u8;
        slice_xzw[i][k][l] = (noise_xzw * 172.5 + 172.5) as u8;
        slice_xyw[i][j][l] = (noise_xyw * 172.5 + 172.5) as u8;
        slice_xyz[i][j][k] = (noise_xyz * 172.5 + 172.5) as u8;
    }
    let slices = [slice_yzw, slice_xzw, slice_xyw, slice_xyz];
    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    let frame_w = 4 * size as u32;
    let frame_h = size as u32;
    for t in 0..size {
        let channel = [frame_h, frame_w]
            .iter()
            .map(|&dim_size| 0..dim_size)
            .multi_cartesian_product()
            .map(|p| {
                let p0 = p[0] as usize;
                let p1 = p[1] as usize;
                slices[p1 / size][p1 % size][p0][t]
            })
            .flat_map(|val| std::iter::repeat(val).take(3))
            .collect::<Vec<u8>>();
        encoder
            .encode(&channel, frame_w, frame_h, ColorType::Rgb8)
            .unwrap();
    }
}
