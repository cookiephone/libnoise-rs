use crate::{constants::*, ptable::build_permutation_table};
use std::sync::Once;

struct StaticPermutationTable {
    table: Option<Vec<usize>>,
    seed: Option<u64>,
    sync: Once,
}

static mut PERMUTATION_TABLE: StaticPermutationTable = StaticPermutationTable {
    table: None,
    seed: None,
    sync: Once::new(),
};

pub fn noise1d(seed: u64, x: f64) -> f64 {
    // no transformation into lattice space required, get cube origin
    let i0 = fast_floor(x);
    // input point relative the two simplex vertices
    let x0 = x - i0;
    let x1 = x0 - 1.0;
    // hashed gradient (-1 or 1) directly, safe because this permutation table cannot index out of bounds
    let i0 = i0 as usize % PERMUTATION_TABLE_SIZE;
    let gi0 = unsafe { hash1d(seed, i0) % GRADIENT_LUT_1D_SIZE };
    let gi1 = unsafe { hash1d(seed, i0 + 1) % GRADIENT_LUT_1D_SIZE };
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution1d(x0, gi0) };
    let n1 = unsafe { contribution1d(x1, gi1) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1) * NORMALIZATION_FACTOR_1D
}

pub fn noise2d(seed: u64, x: f64, y: f64) -> f64 {
    // transform into lattice space and floor for cube origin
    let skew = (x + y) * SKEW_FACTOR_2D;
    let is = fast_floor(x + skew);
    let js = fast_floor(y + skew);
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js) * UNSKEW_FACTOR_2D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    // compute middle simplex vector(s) between 0-vector and 1-vector
    let mut i1 = 1;
    let mut j1 = 0;
    if x0 < y0 {
        i1 = 0;
        j1 = 1;
    }
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + UNSKEW_FACTOR_2D;
    let y1 = y0 - j1 as f64 + UNSKEW_FACTOR_2D;
    let x2 = x0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    let y2 = y0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    // hashed gradient indices, safe because this permutation table cannot index out of bounds
    let is = is as usize % PERMUTATION_TABLE_SIZE;
    let js = js as usize % PERMUTATION_TABLE_SIZE;
    let gi0 = unsafe { hash2d(seed, is, js) } % GRADIENT_LUT_2D_SIZE;
    let gi1 = unsafe { hash2d(seed, is + i1, js + j1) } % GRADIENT_LUT_2D_SIZE;
    let gi2 = unsafe { hash2d(seed, is + 1, js + 1) } % GRADIENT_LUT_2D_SIZE;
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution2d(x0, y0, gi0) };
    let n1 = unsafe { contribution2d(x1, y1, gi1) };
    let n2 = unsafe { contribution2d(x2, y2, gi2) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2) * NORMALIZATION_FACTOR_2D
}

fn fast_floor(x: f64) -> f64 {
    let x_int = x as i64;
    x_int as f64 - (x < x_int as f64) as i32 as f64
}

unsafe fn hash1d(seed: u64, i: usize) -> usize {
    let perm = get_permutation_table(seed);
    *perm.get_unchecked(i)
}

unsafe fn hash2d(seed: u64, i: usize, j: usize) -> usize {
    let perm = get_permutation_table(seed);
    *perm.get_unchecked(i + perm.get_unchecked(j))
}

unsafe fn contribution1d(x: f64, gi: usize) -> f64 {
    if x.abs() >= 0.5_f64.sqrt() {
        0.0
    } else {
        let mut t = R_SQUARED - x * x;
        t *= t;
        t * t * GRADIENT_LUT_1D.get_unchecked(gi) * x
    }
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let mut t = R_SQUARED - x * x - y * y;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = GRADIENT_LUT_2D.get_unchecked(gi);
        t *= t;
        t * t * (gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y)
    }
}

fn get_permutation_table(seed: u64) -> &'static Vec<usize> {
    unsafe {
        if PERMUTATION_TABLE
            .seed
            .is_some_and(|old_seed| old_seed != seed)
        {
            PERMUTATION_TABLE.sync = Once::new();
        }
        PERMUTATION_TABLE.sync.call_once(|| {
            PERMUTATION_TABLE.seed = Some(seed);
            PERMUTATION_TABLE.table =
                Some(build_permutation_table(seed, PERMUTATION_TABLE_SIZE, true));
        });
        PERMUTATION_TABLE.table.as_ref().unwrap()
    }
}
