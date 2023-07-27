use image::GrayImage;
use itertools::Itertools;

#[test]
fn test() {
    let (w, h) = (300, 300);
    let mut data = vec![vec![0.0; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        data[i][j] = noise::simplex::noise2d(0, i as f64 * 0.05, j as f64 * 0.05);
    }
    let data = data.into_iter().flatten().collect::<Vec<f64>>();
    let data = data
        .iter()
        .map(|&val| (127.5 + val * 127.5) as u8)
        .collect::<Vec<u8>>();
    let image = GrayImage::from_raw(w as u32, h as u32, data).unwrap();
    image.save("./img.png").unwrap();
}
