use super::constants::*;
use crate::core::utils::{
    math::{Vec2, Vec3, Vec4},
    ptable::PermutationTable,
};

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dx = x - x0;
    let dxs = smoothstep_3(dx);
    // get sign from hashes
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let sign0 = ((unsafe { perm.hash1d(x0) } % 2) as f64).mul_add(2.0, -1.0);
    let sign1 = ((unsafe { perm.hash1d(x0 + 1) } % 2) as f64).mul_add(2.0, -1.0);
    // compute contributions
    let n0 = sign0 * dx;
    let n1 = sign1 * (dx - 1.0);
    // interpolate values from hypercube corners
    lerp(n0, n1, dxs) * 2.0
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = Vec2::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dx = x - x0;
    let dxs = dx.map(smoothstep_3);
    // hashed gradient indices
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi00 = unsafe { perm.hash2d(x0.x, x0.y) } % CORNERPOINT_GRADIENT_LUT_2D_SIZE;
    let gi01 = unsafe { perm.hash2d(x0.x, x0.y + 1) } % CORNERPOINT_GRADIENT_LUT_2D_SIZE;
    let gi10 = unsafe { perm.hash2d(x0.x + 1, x0.y) } % CORNERPOINT_GRADIENT_LUT_2D_SIZE;
    let gi11 = unsafe { perm.hash2d(x0.x + 1, x0.y + 1) } % CORNERPOINT_GRADIENT_LUT_2D_SIZE;
    // compute contributions
    let n00 = unsafe { contribution2d(dx.x, dx.y, gi00) };
    let n01 = unsafe { contribution2d(dx.x, dx.y - 1.0, gi01) };
    let n10 = unsafe { contribution2d(dx.x - 1.0, dx.y, gi10) };
    let n11 = unsafe { contribution2d(dx.x - 1.0, dx.y - 1.0, gi11) };
    // interpolate values from hypercube corners
    let xn0 = lerp(n00, n10, dxs.x);
    let xn1 = lerp(n01, n11, dxs.x);
    lerp(xn0, xn1, dxs.y)
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = Vec3::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dx = x - x0;
    let dxs = dx.map(smoothstep_3);
    // hashed gradient indices
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi000 = unsafe { perm.hash3d(x0.x, x0.y, x0.z) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi001 = unsafe { perm.hash3d(x0.x, x0.y, x0.z + 1) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi010 = unsafe { perm.hash3d(x0.x, x0.y + 1, x0.z) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi011 = unsafe { perm.hash3d(x0.x, x0.y + 1, x0.z + 1) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi100 = unsafe { perm.hash3d(x0.x + 1, x0.y, x0.z) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi101 = unsafe { perm.hash3d(x0.x + 1, x0.y, x0.z + 1) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi110 = unsafe { perm.hash3d(x0.x + 1, x0.y + 1, x0.z) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    let gi111 =
        unsafe { perm.hash3d(x0.x + 1, x0.y + 1, x0.z + 1) } % CORNERPOINT_GRADIENT_LUT_3D_SIZE;
    // compute contributions
    let n000 = unsafe { contribution3d(dx.x, dx.y, dx.z, gi000) };
    let n001 = unsafe { contribution3d(dx.x, dx.y, dx.z - 1.0, gi001) };
    let n010 = unsafe { contribution3d(dx.x, dx.y - 1.0, dx.z, gi010) };
    let n011 = unsafe { contribution3d(dx.x, dx.y - 1.0, dx.z - 1.0, gi011) };
    let n100 = unsafe { contribution3d(dx.x - 1.0, dx.y, dx.z, gi100) };
    let n101 = unsafe { contribution3d(dx.x - 1.0, dx.y, dx.z - 1.0, gi101) };
    let n110 = unsafe { contribution3d(dx.x - 1.0, dx.y - 1.0, dx.z, gi110) };
    let n111 = unsafe { contribution3d(dx.x - 1.0, dx.y - 1.0, dx.z - 1.0, gi111) };
    // interpolate values from hypercube corners
    let xn00 = lerp(n000, n100, dxs.x);
    let xn01 = lerp(n001, n101, dxs.x);
    let xn10 = lerp(n010, n110, dxs.x);
    let xn11 = lerp(n011, n111, dxs.x);
    let yn0 = lerp(xn00, xn10, dxs.y);
    let yn1 = lerp(xn01, xn11, dxs.y);
    lerp(yn0, yn1, dxs.z) * 0.6666666666666666
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = Vec4::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dx = x - x0;
    let dxs = dx.map(smoothstep_3);
    // hashed gradient indices
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi0000 = unsafe { perm.hash4d(x0.x, x0.y, x0.z, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0001 =
        unsafe { perm.hash4d(x0.x, x0.y, x0.z, x0.w + 1) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0010 =
        unsafe { perm.hash4d(x0.x, x0.y, x0.z + 1, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0011 =
        unsafe { perm.hash4d(x0.x, x0.y, x0.z + 1, x0.w + 1) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0100 =
        unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0101 =
        unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z, x0.w + 1) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0110 =
        unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z + 1, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0111 = unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z + 1, x0.w + 1) }
        % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1000 =
        unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1001 =
        unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z, x0.w + 1) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1010 =
        unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z + 1, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1011 = unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z + 1, x0.w + 1) }
        % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1100 =
        unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z, x0.w) } % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1101 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z, x0.w + 1) }
        % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1110 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z + 1, x0.w) }
        % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1111 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z + 1, x0.w + 1) }
        % CORNERPOINT_GRADIENT_LUT_4D_SIZE;
    // compute contributions
    let n0000 = unsafe { contribution4d(dx.x, dx.y, dx.z, dx.w, gi0000) };
    let n0001 = unsafe { contribution4d(dx.x, dx.y, dx.z, dx.w - 1.0, gi0001) };
    let n0010 = unsafe { contribution4d(dx.x, dx.y, dx.z - 1.0, dx.w, gi0010) };
    let n0011 = unsafe { contribution4d(dx.x, dx.y, dx.z - 1.0, dx.w - 1.0, gi0011) };
    let n0100 = unsafe { contribution4d(dx.x, dx.y - 1.0, dx.z, dx.w, gi0100) };
    let n0101 = unsafe { contribution4d(dx.x, dx.y - 1.0, dx.z, dx.w - 1.0, gi0101) };
    let n0110 = unsafe { contribution4d(dx.x, dx.y - 1.0, dx.z - 1.0, dx.w, gi0110) };
    let n0111 = unsafe { contribution4d(dx.x, dx.y - 1.0, dx.z - 1.0, dx.w - 1.0, gi0111) };
    let n1000 = unsafe { contribution4d(dx.x - 1.0, dx.y, dx.z, dx.w, gi1000) };
    let n1001 = unsafe { contribution4d(dx.x - 1.0, dx.y, dx.z, dx.w - 1.0, gi1001) };
    let n1010 = unsafe { contribution4d(dx.x - 1.0, dx.y, dx.z - 1.0, dx.w, gi1010) };
    let n1011 = unsafe { contribution4d(dx.x - 1.0, dx.y, dx.z - 1.0, dx.w - 1.0, gi1011) };
    let n1100 = unsafe { contribution4d(dx.x - 1.0, dx.y - 1.0, dx.z, dx.w, gi1100) };
    let n1101 = unsafe { contribution4d(dx.x - 1.0, dx.y - 1.0, dx.z, dx.w - 1.0, gi1101) };
    let n1110 = unsafe { contribution4d(dx.x - 1.0, dx.y - 1.0, dx.z - 1.0, dx.w, gi1110) };
    let n1111 = unsafe { contribution4d(dx.x - 1.0, dx.y - 1.0, dx.z - 1.0, dx.w - 1.0, gi1111) };
    // interpolate values from hypercube corners
    let xn000 = lerp(n0000, n1000, dxs.x);
    let xn001 = lerp(n0001, n1001, dxs.x);
    let xn010 = lerp(n0010, n1010, dxs.x);
    let xn011 = lerp(n0011, n1011, dxs.x);
    let xn100 = lerp(n0100, n1100, dxs.x);
    let xn101 = lerp(n0101, n1101, dxs.x);
    let xn110 = lerp(n0110, n1110, dxs.x);
    let xn111 = lerp(n0111, n1111, dxs.x);
    let yn00 = lerp(xn000, xn100, dxs.y);
    let yn01 = lerp(xn001, xn101, dxs.y);
    let yn10 = lerp(xn010, xn110, dxs.y);
    let yn11 = lerp(xn011, xn111, dxs.y);
    let zn0 = lerp(yn00, yn10, dxs.z);
    let zn1 = lerp(yn01, yn11, dxs.z);
    lerp(zn0, zn1, dxs.w) * 0.6664701256514842
}

#[inline]
fn smoothstep_3(t: f64) -> f64 {
    t * t * (t * (-2.0) + 3.0)
}

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let gradient = CORNERPOINT_GRADIENT_LUT_2D.get_unchecked(gi);
    gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y
}

unsafe fn contribution3d(x: f64, y: f64, z: f64, gi: usize) -> f64 {
    let gradient = CORNERPOINT_GRADIENT_LUT_3D.get_unchecked(gi);
    gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y + gradient.get_unchecked(2) * z
}

unsafe fn contribution4d(x: f64, y: f64, z: f64, w: f64, gi: usize) -> f64 {
    let gradient = CORNERPOINT_GRADIENT_LUT_4D.get_unchecked(gi);
    gradient.get_unchecked(0) * x
        + gradient.get_unchecked(1) * y
        + gradient.get_unchecked(2) * z
        + gradient.get_unchecked(3) * w
}
