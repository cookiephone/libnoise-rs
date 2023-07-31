use itertools::Itertools;

pub(crate) fn tensor_indices(shape: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    shape
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
}

pub(crate) fn fast_floor(x: f64) -> f64 {
    let x_int = x as i64;
    x_int as f64 - (x < x_int as f64) as i32 as f64
}

pub(crate) fn fast_floor_usize(x: f64) -> usize {
    let x_int = x as usize;
    x_int - (x < x_int as f64) as usize
}
