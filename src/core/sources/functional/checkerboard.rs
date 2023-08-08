pub(crate) fn noise1d(point: [f64; 1]) -> f64 {
    ((point[0] as isize & 1) as f64).mul_add(2.0, -1.0)
}

pub(crate) fn noise2d(point: [f64; 2]) -> f64 {
    (((point[0] as isize & 1) ^ (point[1] as isize & 1)) as f64).mul_add(2.0, -1.0)
}

pub(crate) fn noise3d(point: [f64; 3]) -> f64 {
    (((point[0] as isize & 1) ^ (point[1] as isize & 1) ^ (point[2] as isize & 1)) as f64)
        .mul_add(2.0, -1.0)
}

pub(crate) fn noise4d(point: [f64; 4]) -> f64 {
    (((point[0] as isize & 1)
        ^ (point[1] as isize & 1)
        ^ (point[2] as isize & 1)
        ^ (point[3] as isize & 1)) as f64)
        .mul_add(2.0, -1.0)
}
