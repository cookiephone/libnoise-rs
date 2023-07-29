use image::GrayImage;
use itertools::Itertools;

#[test]
fn test_1d() {
    let (w, h) = (3000, 300);
    let mut data = vec![vec![255; w]; h];
    for j in 0..w {
        let noise = noise::simplex::noise1d(42, j as f64 * 0.013);
        //let noise = noise::simplex::noise2d(42, j as f64 * 0.013, 0.0);
        let noise = (h as f64 * (noise * 0.5 + 0.5)) as usize;
        for (i, row) in data.iter_mut().enumerate() {
            if noise.abs_diff(i) < (h / 30) {
                row[j] = 0;
            }
        }
    }
    let data = data.into_iter().flatten().collect::<Vec<u8>>();
    let image = GrayImage::from_raw(w as u32, h as u32, data).unwrap();
    image.save("./img.png").unwrap();
}

#[test]
fn test_2d() {
    let (w, h) = (3000, 3000);
    let mut data = vec![vec![0.0; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        data[i][j] = noise::simplex::noise2d(42, i as f64 * 0.013, j as f64 * 0.013);
    }
    let data = data.into_iter().flatten().collect::<Vec<f64>>();
    let data = data
        .iter()
        .map(|&val| (127.5 + val * 127.5) as u8)
        .collect::<Vec<u8>>();
    let image = GrayImage::from_raw(w as u32, h as u32, data).unwrap();
    image.save("./img.png").unwrap();
}
