use crate::{constants::*, ptable::build_permutation_table};
use std::sync::Once;

static mut PERMUTATION_TABLE: (Once, Option<u64>, Option<Vec<usize>>) = (Once::new(), None, None);

fn get_permutation_table(seed: u64) -> &'static Vec<usize> {
    unsafe {
        if PERMUTATION_TABLE.1.is_some_and(|old_seed| old_seed != seed) {
            PERMUTATION_TABLE.0 = Once::new();
        }
        PERMUTATION_TABLE.0.call_once(|| {
            PERMUTATION_TABLE.1 = Some(seed);
            PERMUTATION_TABLE.2 = Some(build_permutation_table(seed, PERMUTATION_TABLE_SIZE, true));
        });
        PERMUTATION_TABLE.2.as_ref().unwrap()
    }
}

pub fn noise2d(seed: u64, x: f64, y: f64) -> f64 {
    // transform into lattice space and floor for cube origin
    let skew = (x + y) * SKEW_FACTOR_2D;
    let is = (x + skew).floor();
    let js = (y + skew).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js) * UNSKEW_FACTOR_2D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    // compute middle simplex vector(s) between 0-vector and 1-vector
    let mut i1 = 1.0;
    let mut j1 = 0.0;
    if x0 < y0 {
        i1 = 0.0;
        j1 = 1.0;
    }
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 + UNSKEW_FACTOR_2D;
    let y1 = y0 - j1 + UNSKEW_FACTOR_2D;
    let x2 = x0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    let y2 = y0 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
    // hashed gradient indices, safe because this permutation table cannot index out of bounds
    let is = is as usize % 256;
    let js = js as usize % 256;
    let gi0 = unsafe { hash2d(seed, is, js) };// unsafe { hash![is + hash![js]] % 4 };
    let gi1 = unsafe { hash2d(seed, is + i1 as usize, js + j1 as usize) };// unsafe { hash![is + i1 as usize + hash![js + j1 as usize]] % 4 };
    let gi2 = unsafe { hash2d(seed, is + 1, js + 1) };// unsafe { hash![is + 1 + hash![js + 1]] % 4 };
    // compute contributions, safe because gradient lookup table is known
    let n0 = unsafe { contribution2d(x0, y0, gi0) };
    let n1 = unsafe { contribution2d(x1, y1, gi1) };
    let n2 = unsafe { contribution2d(x2, y2, gi2) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2) * 99.83685446303647
}

unsafe fn hash2d(seed: u64, i: usize, j: usize) -> usize {
    let perm = get_permutation_table(seed);
    perm.get_unchecked(i + perm.get_unchecked(j)) % 4
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let mut t = R_SQUARED - x * x - y * y;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = GRADIENT_LUT_2D.get_unchecked(gi);
        t = t * t;
        t * t * (gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y)
    }
}
