use itertools::Itertools;

fn tensor_indices(shape: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    shape
        .iter()
        .map(|&dim_size| 0..dim_size)
        .multi_cartesian_product()
}

pub(crate) fn cartesian_lattice_points(
    shape: &[usize],
    scale: f64,
) -> impl Iterator<Item = Vec<f64>> {
    tensor_indices(shape).map(move |point| {
        point
            .iter()
            .map(|&component| component as f64 * scale)
            .collect()
    })
}
