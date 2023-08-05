use super::constants::*;
use crate::utils::ptable::PermutationTable;

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // no transformation into lattice space required, get cube origin
    let i0 = x.floor();
    // input point relative the two simplex vertices
    let x0 = x - i0;
    let x1 = x0 - 1.0;
    // hashed gradient (-1 or 1) directly
    let i0 = i0.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let gi0 = unsafe { perm.hash1d(i0) % GRADIENT_LUT_1D_SIZE };
    let gi1 = unsafe { perm.hash1d(i0 + 1) % GRADIENT_LUT_1D_SIZE };
    // compute contributions
    let n0 = unsafe { contribution1d(x0, gi0) };
    let n1 = unsafe { contribution1d(x1, gi1) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1) * SIMPLEX_NORMALIZATION_FACTOR_1D
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = point[0];
    let y = point[1];
    // transform into lattice space and floor for cube origin
    let skew = (x + y) * SIMPLEX_SKEW_FACTOR_2D;
    let is = (x + skew).floor();
    let js = (y + skew).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js) * SIMPLEX_UNSKEW_FACTOR_2D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let mut i1 = 1;
    let mut j1 = 0;
    if x0 < y0 {
        i1 = 0;
        j1 = 1;
    }
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + SIMPLEX_UNSKEW_FACTOR_2D;
    let y1 = y0 - j1 as f64 + SIMPLEX_UNSKEW_FACTOR_2D;
    let x2 = x0 - 1.0 + 2.0 * SIMPLEX_UNSKEW_FACTOR_2D;
    let y2 = y0 - 1.0 + 2.0 * SIMPLEX_UNSKEW_FACTOR_2D;
    // hashed gradient indices
    let is = is.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let js = js.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let gi0 = unsafe { perm.hash2d(is, js) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi1 = unsafe { perm.hash2d(is + i1, js + j1) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi2 = unsafe { perm.hash2d(is + 1, js + 1) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution2d(x0, y0, gi0) };
    let n1 = unsafe { contribution2d(x1, y1, gi1) };
    let n2 = unsafe { contribution2d(x2, y2, gi2) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2) * SIMPLEX_NORMALIZATION_FACTOR_2D
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    // transform into lattice space and floor for cube origin
    let skew = (x + y + z) * SIMPLEX_SKEW_FACTOR_3D;
    let is = (x + skew).floor();
    let js = (y + skew).floor();
    let ks = (z + skew).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js + ks) * SIMPLEX_UNSKEW_FACTOR_3D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    let z0 = z - ks + unskew;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let idx = (x0 > y0) as usize * 4 + (y0 > z0) as usize * 2 + (x0 > z0) as usize;
    let i1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][0];
    let j1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][1];
    let k1 = SIMPLEX_TRAVERSAL_LUT_3D[idx][2];
    let i2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][3];
    let j2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][4];
    let k2 = SIMPLEX_TRAVERSAL_LUT_3D[idx][5];
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + SIMPLEX_UNSKEW_FACTOR_3D;
    let y1 = y0 - j1 as f64 + SIMPLEX_UNSKEW_FACTOR_3D;
    let z1 = z0 - k1 as f64 + SIMPLEX_UNSKEW_FACTOR_3D;
    let x2 = x0 - i2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let y2 = y0 - j2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let z2 = z0 - k2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let x3 = x0 - 1.0 + 3.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let y3 = y0 - 1.0 + 3.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let z3 = z0 - 1.0 + 3.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    // hashed gradient indices
    let is = is.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let js = js.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let ks = ks.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let gi0 = unsafe { perm.hash3d(is, js, ks) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi1 = unsafe { perm.hash3d(is + i1, js + j1, ks + k1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi2 = unsafe { perm.hash3d(is + i2, js + j2, ks + k2) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi3 = unsafe { perm.hash3d(is + 1, js + 1, ks + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution3d(x0, y0, z0, gi0) };
    let n1 = unsafe { contribution3d(x1, y1, z1, gi1) };
    let n2 = unsafe { contribution3d(x2, y2, z2, gi2) };
    let n3 = unsafe { contribution3d(x3, y3, z3, gi3) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2 + n3) * SIMPLEX_NORMALIZATION_FACTOR_3D
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    let w = point[3];
    // transform into lattice space and floor for cube origin
    let skew = (x + y + z + w) * SIMPLEX_SKEW_FACTOR_4D;
    let is = (x + skew).floor();
    let js = (y + skew).floor();
    let ks = (z + skew).floor();
    let ls = (w + skew).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let unskew = (is + js + ks + ls) * SIMPLEX_UNSKEW_FACTOR_4D;
    let x0 = x - is + unskew;
    let y0 = y - js + unskew;
    let z0 = z - ks + unskew;
    let w0 = w - ls + unskew;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let idx = (x0 > y0) as usize * 32
        + (x0 > z0) as usize * 16
        + (y0 > z0) as usize * 8
        + (x0 > w0) as usize * 4
        + (y0 > w0) as usize * 2
        + (z0 > w0) as usize;
    let i1 = SIMPLEX_TRAVERSAL_LUT_4D[idx][0];
    let j1 = SIMPLEX_TRAVERSAL_LUT_4D[idx][1];
    let k1 = SIMPLEX_TRAVERSAL_LUT_4D[idx][2];
    let l1 = SIMPLEX_TRAVERSAL_LUT_4D[idx][3];
    let i2 = SIMPLEX_TRAVERSAL_LUT_4D[idx][4];
    let j2 = SIMPLEX_TRAVERSAL_LUT_4D[idx][5];
    let k2 = SIMPLEX_TRAVERSAL_LUT_4D[idx][6];
    let l2 = SIMPLEX_TRAVERSAL_LUT_4D[idx][7];
    let i3 = SIMPLEX_TRAVERSAL_LUT_4D[idx][8];
    let j3 = SIMPLEX_TRAVERSAL_LUT_4D[idx][9];
    let k3 = SIMPLEX_TRAVERSAL_LUT_4D[idx][10];
    let l3 = SIMPLEX_TRAVERSAL_LUT_4D[idx][11];
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1 as f64 + SIMPLEX_UNSKEW_FACTOR_4D;
    let y1 = y0 - j1 as f64 + SIMPLEX_UNSKEW_FACTOR_4D;
    let z1 = z0 - k1 as f64 + SIMPLEX_UNSKEW_FACTOR_4D;
    let w1 = w0 - l1 as f64 + SIMPLEX_UNSKEW_FACTOR_4D;
    let x2 = x0 - i2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let y2 = y0 - j2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let z2 = z0 - k2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let w2 = w0 - l2 as f64 + 2.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let x3 = x0 - i3 as f64 + 3.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let y3 = y0 - j3 as f64 + 3.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let z3 = z0 - k3 as f64 + 3.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let w3 = w0 - l3 as f64 + 3.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let x4 = x0 - 1.0 + 4.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let y4 = y0 - 1.0 + 4.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let z4 = z0 - 1.0 + 4.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let w4 = w0 - 1.0 + 4.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    // hashed gradient indices
    let is = is.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let js = js.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let ks = ks.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let ls = ls.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let gi0 = unsafe { perm.hash4d(is, js, ks, ls) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1 =
        unsafe { perm.hash4d(is + i1, js + j1, ks + k1, ls + l1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi2 =
        unsafe { perm.hash4d(is + i2, js + j2, ks + k2, ls + l2) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi3 =
        unsafe { perm.hash4d(is + i3, js + j3, ks + k3, ls + l3) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi4 =
        unsafe { perm.hash4d(is + 1, js + 1, ks + 1, ls + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution4d(x0, y0, z0, w0, gi0) };
    let n1 = unsafe { contribution4d(x1, y1, z1, w1, gi1) };
    let n2 = unsafe { contribution4d(x2, y2, z2, w2, gi2) };
    let n3 = unsafe { contribution4d(x3, y3, z3, w3, gi3) };
    let n4 = unsafe { contribution4d(x4, y4, z4, w4, gi4) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2 + n3 + n4) * SIMPLEX_NORMALIZATION_FACTOR_4D
}

unsafe fn contribution1d(x: f64, gi: usize) -> f64 {
    if x.abs() >= std::f64::consts::FRAC_1_SQRT_2 {
        0.0
    } else {
        let mut t = SIMPLEX_R_SQUARED - x * x;
        t *= t;
        t * t * GRADIENT_LUT_1D.get_unchecked(gi) * x
    }
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x * x - y * y;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_2D.get_unchecked(gi);
        t *= t;
        t * t * (gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y)
    }
}

unsafe fn contribution3d(x: f64, y: f64, z: f64, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x * x - y * y - z * z;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_3D.get_unchecked(gi);
        t *= t;
        t * t
            * (gradient.get_unchecked(0) * x
                + gradient.get_unchecked(1) * y
                + gradient.get_unchecked(2) * z)
    }
}

unsafe fn contribution4d(x: f64, y: f64, z: f64, w: f64, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x * x - y * y - z * z - w * w;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_4D.get_unchecked(gi);
        t *= t;
        t * t
            * (gradient.get_unchecked(0) * x
                + gradient.get_unchecked(1) * y
                + gradient.get_unchecked(2) * z
                + gradient.get_unchecked(3) * w)
    }
}
