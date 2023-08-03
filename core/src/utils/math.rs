use itertools::Itertools;

pub(crate) fn tensor_indices(shape: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    shape
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
}

// this is significantly faster than fast_floor, but not portable
/*
fn floor_asm() {
    let mut x = black_box(69348.9253_f64);
    unsafe {
        asm!(
            "vroundsd xmm0, xmm0, xmm0, 9",
            inout("xmm0") x,
        )
    }
}
*/

pub(crate) fn fast_floor(x: f64) -> f64 {
    let x_int = x as i64;
    x_int as f64 - (x < x_int as f64) as i32 as f64
}
