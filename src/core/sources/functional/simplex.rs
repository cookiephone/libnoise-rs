use super::constants::*;
use crate::core::utils::{
    math::{Vec2, Vec3, Vec4},
    ptable::PermutationTable,
};

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
    let x = Vec2::from(point);
    // transform into lattice space and floor for cube origin
    let is = (x + x.sum() * SIMPLEX_SKEW_FACTOR_2D).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let x0 = x - is + is.sum() * SIMPLEX_UNSKEW_FACTOR_2D;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let mut i1 = Vec2::from([1, 0]);
    if x0.x < x0.y {
        i1.x = 0;
        i1.y = 1;
    }
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1.cast() + SIMPLEX_UNSKEW_FACTOR_2D;
    let x2 = x0 - 1.0 + 2.0 * SIMPLEX_UNSKEW_FACTOR_2D;
    // hashed gradient indices
    let is = is.rem_euclid(PERMUTATION_TABLE_SIZE as f64).cast();
    let gi0 = unsafe { perm.hash2d(is.x, is.y) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi1 = unsafe { perm.hash2d(is.x + i1.x, is.y + i1.y) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi2 = unsafe { perm.hash2d(is.x + 1, is.y + 1) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution2d(x0, gi0) };
    let n1 = unsafe { contribution2d(x1, gi1) };
    let n2 = unsafe { contribution2d(x2, gi2) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2) * SIMPLEX_NORMALIZATION_FACTOR_2D
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = Vec3::from(point);
    // transform into lattice space and floor for cube origin
    let is = (x + x.sum() * SIMPLEX_SKEW_FACTOR_3D).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let x0 = x - is + is.sum() * SIMPLEX_UNSKEW_FACTOR_3D;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let idx = (x0.x > x0.y) as usize * 4 + (x0.y > x0.z) as usize * 2 + (x0.x > x0.z) as usize;
    let i1 = Vec3::from([
        SIMPLEX_TRAVERSAL_LUT_3D[idx][0],
        SIMPLEX_TRAVERSAL_LUT_3D[idx][1],
        SIMPLEX_TRAVERSAL_LUT_3D[idx][2],
    ]);
    let i2 = Vec3::from([
        SIMPLEX_TRAVERSAL_LUT_3D[idx][3],
        SIMPLEX_TRAVERSAL_LUT_3D[idx][4],
        SIMPLEX_TRAVERSAL_LUT_3D[idx][5],
    ]);
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1.cast() + SIMPLEX_UNSKEW_FACTOR_3D;
    let x2 = x0 - i2.cast() + 2.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    let x3 = x0 - 1.0 + 3.0 * SIMPLEX_UNSKEW_FACTOR_3D;
    // hashed gradient indices
    let is = is.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi0 = unsafe { perm.hash3d_vec(is) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi1 = unsafe { perm.hash3d_vec(is + i1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi2 = unsafe { perm.hash3d_vec(is + i2) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi3 = unsafe { perm.hash3d_vec(is + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution3d(x0, gi0) };
    let n1 = unsafe { contribution3d(x1, gi1) };
    let n2 = unsafe { contribution3d(x2, gi2) };
    let n3 = unsafe { contribution3d(x3, gi3) };
    // combine contributions and scale to [-1, 1]
    (n0 + n1 + n2 + n3) * SIMPLEX_NORMALIZATION_FACTOR_3D
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = Vec4::from(point);
    // transform into lattice space and floor for cube origin
    let is = (x + x.sum() * SIMPLEX_SKEW_FACTOR_4D).floor();
    // input point relative to unskewed cube (and simplex) origin in source space
    let x0 = x - is + is.sum() * SIMPLEX_UNSKEW_FACTOR_4D;
    // compute middle simplex traversal vector(s) between 0-vector and 1-vector
    let idx = (x0.x > x0.y) as usize * 32
        + (x0.x > x0.z) as usize * 16
        + (x0.y > x0.z) as usize * 8
        + (x0.x > x0.w) as usize * 4
        + (x0.y > x0.w) as usize * 2
        + (x0.z > x0.w) as usize;
    let i1 = Vec4::from([
        SIMPLEX_TRAVERSAL_LUT_4D[idx][0],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][1],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][2],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][3],
    ]);
    let i2 = Vec4::from([
        SIMPLEX_TRAVERSAL_LUT_4D[idx][4],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][5],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][6],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][7],
    ]);
    let i3 = Vec4::from([
        SIMPLEX_TRAVERSAL_LUT_4D[idx][8],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][9],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][10],
        SIMPLEX_TRAVERSAL_LUT_4D[idx][11],
    ]);
    // imput point relative to other unskewed simplex vertices
    let x1 = x0 - i1.cast() + SIMPLEX_UNSKEW_FACTOR_4D;
    let x2 = x0 - i2.cast() + 2.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let x3 = x0 - i3.cast() + 3.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    let x4 = x0 - 1.0 + 4.0 * SIMPLEX_UNSKEW_FACTOR_4D;
    // hashed gradient indices
    let is = is.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi0 = unsafe { perm.hash4d_vec(is) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1 = unsafe { perm.hash4d_vec(is + i1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi2 = unsafe { perm.hash4d_vec(is + i2) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi3 = unsafe { perm.hash4d_vec(is + i3) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi4 = unsafe { perm.hash4d_vec(is + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    // compute contributions
    let n0 = unsafe { contribution4d(x0, gi0) };
    let n1 = unsafe { contribution4d(x1, gi1) };
    let n2 = unsafe { contribution4d(x2, gi2) };
    let n3 = unsafe { contribution4d(x3, gi3) };
    let n4 = unsafe { contribution4d(x4, gi4) };
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

unsafe fn contribution2d(x: Vec2<f64>, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x.x * x.x - x.y * x.y;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_2D.get_unchecked(gi);
        t *= t;
        t * t * (gradient.get_unchecked(0) * x.x + gradient.get_unchecked(1) * x.y)
    }
}

unsafe fn contribution3d(x: Vec3<f64>, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x.x * x.x - x.y * x.y - x.z * x.z;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_3D.get_unchecked(gi);
        t *= t;
        t * t
            * (gradient.get_unchecked(0) * x.x
                + gradient.get_unchecked(1) * x.y
                + gradient.get_unchecked(2) * x.z)
    }
}

unsafe fn contribution4d(x: Vec4<f64>, gi: usize) -> f64 {
    let mut t = SIMPLEX_R_SQUARED - x.x * x.x - x.y * x.y - x.z * x.z - x.w * x.w;
    if t <= 0.0 {
        0.0
    } else {
        let gradient = MIDPOINT_GRADIENT_LUT_4D.get_unchecked(gi);
        t *= t;
        t * t
            * (gradient.get_unchecked(0) * x.x
                + gradient.get_unchecked(1) * x.y
                + gradient.get_unchecked(2) * x.z
                + gradient.get_unchecked(3) * x.w)
    }
}
