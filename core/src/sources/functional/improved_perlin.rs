use super::constants::*;
use crate::utils::ptable::PermutationTable;

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // origin of hypercube in which input lies
    let x0 = x.floor() as usize;
    // smoothed distance from hypercube origin
    let dx = x - x0 as f64;
    let dxs = smoothstep_5(dx);
    // get sign from hashes
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let sign0 = ((unsafe { perm.hash1d(x0) } % 2) as f64).mul_add(2.0, -1.0);
    let sign1 = ((unsafe { perm.hash1d(x0 + 1) } % 2) as f64).mul_add(2.0, -1.0);
    // compute contributions
    let n0 = sign0 * dx;
    let n1 = sign1 * (dx - 1.0);
    // interpolate values from hypercube corners
    lerp(n0, n1, dxs) * 2.0
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = point[0];
    let y = point[1];
    // origin of hypercube in which input lies
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    // smoothed distance from hypercube origin
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let dxs = smoothstep_5(dx);
    let dys = smoothstep_5(dy);
    // hashed gradient indices
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi00 = unsafe { perm.hash2d(x0, y0) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi01 = unsafe { perm.hash2d(x0, y0 + 1) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi10 = unsafe { perm.hash2d(x0 + 1, y0) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    let gi11 = unsafe { perm.hash2d(x0 + 1, y0 + 1) } % MIDPOINT_GRADIENT_LUT_2D_SIZE;
    // compute contributions
    let n00 = unsafe { contribution2d(dx, dy, gi00) };
    let n01 = unsafe { contribution2d(dx, dy - 1.0, gi01) };
    let n10 = unsafe { contribution2d(dx - 1.0, dy, gi10) };
    let n11 = unsafe { contribution2d(dx - 1.0, dy - 1.0, gi11) };
    // interpolate values from hypercube corners
    let xn0 = lerp(n00, n10, dxs);
    let xn1 = lerp(n01, n11, dxs);
    lerp(xn0, xn1, dys) * 1.868202396614395
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    // origin of hypercube in which input lies
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let z0 = z.floor() as usize;
    // smoothed distance from hypercube origin
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let dz = z - z0 as f64;
    let dxs = smoothstep_5(dx);
    let dys = smoothstep_5(dy);
    let dzs = smoothstep_5(dz);
    // hashed gradient indices
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let z0 = z0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi000 = unsafe { perm.hash3d(x0, y0, z0) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi001 = unsafe { perm.hash3d(x0, y0, z0 + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi010 = unsafe { perm.hash3d(x0, y0 + 1, z0) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi011 = unsafe { perm.hash3d(x0, y0 + 1, z0 + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi100 = unsafe { perm.hash3d(x0 + 1, y0, z0) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi101 = unsafe { perm.hash3d(x0 + 1, y0, z0 + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi110 = unsafe { perm.hash3d(x0 + 1, y0 + 1, z0) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    let gi111 = unsafe { perm.hash3d(x0 + 1, y0 + 1, z0 + 1) } % MIDPOINT_GRADIENT_LUT_3D_SIZE;
    // compute contributions
    let n000 = unsafe { contribution3d(dx, dy, dz, gi000) };
    let n001 = unsafe { contribution3d(dx, dy, dz - 1.0, gi001) };
    let n010 = unsafe { contribution3d(dx, dy - 1.0, dz, gi010) };
    let n011 = unsafe { contribution3d(dx, dy - 1.0, dz - 1.0, gi011) };
    let n100 = unsafe { contribution3d(dx - 1.0, dy, dz, gi100) };
    let n101 = unsafe { contribution3d(dx - 1.0, dy, dz - 1.0, gi101) };
    let n110 = unsafe { contribution3d(dx - 1.0, dy - 1.0, dz, gi110) };
    let n111 = unsafe { contribution3d(dx - 1.0, dy - 1.0, dz - 1.0, gi111) };
    // interpolate values from hypercube corners
    let xn00 = lerp(n000, n100, dxs);
    let xn01 = lerp(n001, n101, dxs);
    let xn10 = lerp(n010, n110, dxs);
    let xn11 = lerp(n011, n111, dxs);
    let yn0 = lerp(xn00, xn10, dys);
    let yn1 = lerp(xn01, xn11, dys);
    lerp(yn0, yn1, dzs) * 0.9714130038529027
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    let w = point[3];
    // origin of hypercube in which input lies
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let z0 = z.floor() as usize;
    let w0 = w.floor() as usize;
    // smoothed distance from hypercube origin
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let dz = z - z0 as f64;
    let dw = w - w0 as f64;
    let dxs = smoothstep_5(dx);
    let dys = smoothstep_5(dy);
    let dzs = smoothstep_5(dz);
    let dws = smoothstep_5(dw);
    // hashed gradient indices
    let x0 = x0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let y0 = y0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let z0 = z0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let w0 = w0.rem_euclid(PERMUTATION_TABLE_SIZE);
    let gi0000 = unsafe { perm.hash4d(x0, y0, z0, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0001 = unsafe { perm.hash4d(x0, y0, z0, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0010 = unsafe { perm.hash4d(x0, y0, z0 + 1, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0011 = unsafe { perm.hash4d(x0, y0, z0 + 1, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0100 = unsafe { perm.hash4d(x0, y0 + 1, z0, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0101 = unsafe { perm.hash4d(x0, y0 + 1, z0, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0110 = unsafe { perm.hash4d(x0, y0 + 1, z0 + 1, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi0111 = unsafe { perm.hash4d(x0, y0 + 1, z0 + 1, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1000 = unsafe { perm.hash4d(x0 + 1, y0, z0, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1001 = unsafe { perm.hash4d(x0 + 1, y0, z0, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1010 = unsafe { perm.hash4d(x0 + 1, y0, z0 + 1, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1011 = unsafe { perm.hash4d(x0 + 1, y0, z0 + 1, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1100 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1101 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1110 = unsafe { perm.hash4d(x0 + 1, y0 + 1, z0 + 1, w0) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    let gi1111 =
        unsafe { perm.hash4d(x0 + 1, y0 + 1, z0 + 1, w0 + 1) } % MIDPOINT_GRADIENT_LUT_4D_SIZE;
    // compute contributions
    let n0000 = unsafe { contribution4d(dx, dy, dz, dw, gi0000) };
    let n0001 = unsafe { contribution4d(dx, dy, dz, dw - 1.0, gi0001) };
    let n0010 = unsafe { contribution4d(dx, dy, dz - 1.0, dw, gi0010) };
    let n0011 = unsafe { contribution4d(dx, dy, dz - 1.0, dw - 1.0, gi0011) };
    let n0100 = unsafe { contribution4d(dx, dy - 1.0, dz, dw, gi0100) };
    let n0101 = unsafe { contribution4d(dx, dy - 1.0, dz, dw - 1.0, gi0101) };
    let n0110 = unsafe { contribution4d(dx, dy - 1.0, dz - 1.0, dw, gi0110) };
    let n0111 = unsafe { contribution4d(dx, dy - 1.0, dz - 1.0, dw - 1.0, gi0111) };
    let n1000 = unsafe { contribution4d(dx - 1.0, dy, dz, dw, gi1000) };
    let n1001 = unsafe { contribution4d(dx - 1.0, dy, dz, dw - 1.0, gi1001) };
    let n1010 = unsafe { contribution4d(dx - 1.0, dy, dz - 1.0, dw, gi1010) };
    let n1011 = unsafe { contribution4d(dx - 1.0, dy, dz - 1.0, dw - 1.0, gi1011) };
    let n1100 = unsafe { contribution4d(dx - 1.0, dy - 1.0, dz, dw, gi1100) };
    let n1101 = unsafe { contribution4d(dx - 1.0, dy - 1.0, dz, dw - 1.0, gi1101) };
    let n1110 = unsafe { contribution4d(dx - 1.0, dy - 1.0, dz - 1.0, dw, gi1110) };
    let n1111 = unsafe { contribution4d(dx - 1.0, dy - 1.0, dz - 1.0, dw - 1.0, gi1111) };
    // interpolate values from hypercube corners
    let xn000 = lerp(n0000, n1000, dxs);
    let xn001 = lerp(n0001, n1001, dxs);
    let xn010 = lerp(n0010, n1010, dxs);
    let xn011 = lerp(n0011, n1011, dxs);
    let xn100 = lerp(n0100, n1100, dxs);
    let xn101 = lerp(n0101, n1101, dxs);
    let xn110 = lerp(n0110, n1110, dxs);
    let xn111 = lerp(n0111, n1111, dxs);
    let yn00 = lerp(xn000, xn100, dys);
    let yn01 = lerp(xn001, xn101, dys);
    let yn10 = lerp(xn010, xn110, dys);
    let yn11 = lerp(xn011, xn111, dys);
    let zn0 = lerp(yn00, yn10, dzs);
    let zn1 = lerp(yn01, yn11, dzs);
    lerp(zn0, zn1, dws) * 0.7521488407111554
}

fn smoothstep_5(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

unsafe fn contribution2d(x: f64, y: f64, gi: usize) -> f64 {
    let gradient = MIDPOINT_GRADIENT_LUT_2D.get_unchecked(gi);
    gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y
}

unsafe fn contribution3d(x: f64, y: f64, z: f64, gi: usize) -> f64 {
    let gradient = MIDPOINT_GRADIENT_LUT_3D.get_unchecked(gi);
    gradient.get_unchecked(0) * x + gradient.get_unchecked(1) * y + gradient.get_unchecked(2) * z
}

unsafe fn contribution4d(x: f64, y: f64, z: f64, w: f64, gi: usize) -> f64 {
    let gradient = MIDPOINT_GRADIENT_LUT_4D.get_unchecked(gi);
    gradient.get_unchecked(0) * x
        + gradient.get_unchecked(1) * y
        + gradient.get_unchecked(2) * z
        + gradient.get_unchecked(3) * w
}
