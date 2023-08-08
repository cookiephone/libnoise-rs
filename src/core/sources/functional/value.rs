use super::constants::PERMUTATION_TABLE_SIZE;
use crate::core::utils::{
    math::{Vec2, Vec3, Vec4},
    ptable::PermutationTable,
};

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dxs = smoothstep_3(x - x0);
    // get values from hypercube corners
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE as f64) as usize;
    let f0 = unsafe { perm.hash1d(x0) } as f64;
    let f1 = unsafe { perm.hash1d(x0 + 1) } as f64;
    // interpolate values from hypercube corners
    let xf = lerp(f0, f1, dxs);
    normalize(xf)
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = Vec2::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dxs = (x - x0).map(smoothstep_3);
    // get values from hypercube corners
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let f00 = unsafe { perm.hash2d(x0.x, x0.y) } as f64;
    let f01 = unsafe { perm.hash2d(x0.x, x0.y + 1) } as f64;
    let f10 = unsafe { perm.hash2d(x0.x + 1, x0.y) } as f64;
    let f11 = unsafe { perm.hash2d(x0.x + 1, x0.y + 1) } as f64;
    // interpolate values from hypercube corners
    let xf0 = lerp(f00, f10, dxs.x);
    let xf1 = lerp(f01, f11, dxs.x);
    let yf = lerp(xf0, xf1, dxs.y);
    normalize(yf)
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = Vec3::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dxs = (x - x0).map(smoothstep_3);
    // get values from hypercube corners
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let f000 = unsafe { perm.hash3d(x0.x, x0.y, x0.z) } as f64;
    let f001 = unsafe { perm.hash3d(x0.x, x0.y, x0.z + 1) } as f64;
    let f010 = unsafe { perm.hash3d(x0.x, x0.y + 1, x0.z) } as f64;
    let f011 = unsafe { perm.hash3d(x0.x, x0.y + 1, x0.z + 1) } as f64;
    let f100 = unsafe { perm.hash3d(x0.x + 1, x0.y, x0.z) } as f64;
    let f101 = unsafe { perm.hash3d(x0.x + 1, x0.y, x0.z + 1) } as f64;
    let f110 = unsafe { perm.hash3d(x0.x + 1, x0.y + 1, x0.z) } as f64;
    let f111 = unsafe { perm.hash3d(x0.x + 1, x0.y + 1, x0.z + 1) } as f64;
    // interpolate values from hypercube corners
    let xf00 = lerp(f000, f100, dxs.x);
    let xf01 = lerp(f001, f101, dxs.x);
    let xf10 = lerp(f010, f110, dxs.x);
    let xf11 = lerp(f011, f111, dxs.x);
    let yf0 = lerp(xf00, xf10, dxs.y);
    let yf1 = lerp(xf01, xf11, dxs.y);
    let zf = lerp(yf0, yf1, dxs.z);
    normalize(zf)
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = Vec4::from(point);
    // origin of hypercube in which input lies
    let x0 = x.floor();
    // smoothed distance from hypercube origin
    let dxs = (x - x0).map(smoothstep_3);
    // get values from hypercube corners
    let x0 = x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE);
    let f0000 = unsafe { perm.hash4d(x0.x, x0.y, x0.z, x0.w) } as f64;
    let f0001 = unsafe { perm.hash4d(x0.x, x0.y, x0.z, x0.w + 1) } as f64;
    let f0010 = unsafe { perm.hash4d(x0.x, x0.y, x0.z + 1, x0.w) } as f64;
    let f0011 = unsafe { perm.hash4d(x0.x, x0.y, x0.z + 1, x0.w + 1) } as f64;
    let f0100 = unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z, x0.w) } as f64;
    let f0101 = unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z, x0.w + 1) } as f64;
    let f0110 = unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z + 1, x0.w) } as f64;
    let f0111 = unsafe { perm.hash4d(x0.x, x0.y + 1, x0.z + 1, x0.w + 1) } as f64;
    let f1000 = unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z, x0.w) } as f64;
    let f1001 = unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z, x0.w + 1) } as f64;
    let f1010 = unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z + 1, x0.w) } as f64;
    let f1011 = unsafe { perm.hash4d(x0.x + 1, x0.y, x0.z + 1, x0.w + 1) } as f64;
    let f1100 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z, x0.w) } as f64;
    let f1101 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z, x0.w + 1) } as f64;
    let f1110 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z + 1, x0.w) } as f64;
    let f1111 = unsafe { perm.hash4d(x0.x + 1, x0.y + 1, x0.z + 1, x0.w + 1) } as f64;
    // interpolate values from hypercube corners
    let xf000 = lerp(f0000, f1000, dxs.x);
    let xf001 = lerp(f0001, f1001, dxs.x);
    let xf010 = lerp(f0010, f1010, dxs.x);
    let xf011 = lerp(f0011, f1011, dxs.x);
    let xf100 = lerp(f0100, f1100, dxs.x);
    let xf101 = lerp(f0101, f1101, dxs.x);
    let xf110 = lerp(f0110, f1110, dxs.x);
    let xf111 = lerp(f0111, f1111, dxs.x);
    let yf00 = lerp(xf000, xf100, dxs.y);
    let yf01 = lerp(xf001, xf101, dxs.y);
    let yf10 = lerp(xf010, xf110, dxs.y);
    let yf11 = lerp(xf011, xf111, dxs.y);
    let zf0 = lerp(yf00, yf10, dxs.z);
    let zf1 = lerp(yf01, yf11, dxs.z);
    let wf = lerp(zf0, zf1, dxs.w);
    normalize(wf)
}

#[inline]
fn normalize(x: f64) -> f64 {
    2.0 / PERMUTATION_TABLE_SIZE as f64 * x - 1.0
}

#[inline]
fn smoothstep_3(t: f64) -> f64 {
    t * t * (t * (-2.0) + 3.0)
}

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
