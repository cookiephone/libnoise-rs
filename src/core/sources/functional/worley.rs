use super::constants::PERMUTATION_TABLE_SIZE;
use crate::core::utils::{
    math::{Vec2, Vec3, Vec4},
    ptable::PermutationTable,
};

pub(crate) fn noise1d(perm: &PermutationTable, point: [f64; 1]) -> f64 {
    let x = point[0];
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let dx = x - x0;
    // compute distance to closest neighbor
    let mut min_dist = f64::INFINITY;
    for i in (-1..=1).map(|val| val as f64) {
        let pn = point1d(perm, x0 + i);
        let dn = (pn + i - dx).abs();
        min_dist = min_dist.min(dn);
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist * 2.0 - 1.0
}

pub(crate) fn noise2d(perm: &PermutationTable, point: [f64; 2]) -> f64 {
    let x = Vec2::from(point);
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let dx = x - x0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in -1..=1 {
        for j in -1..=1 {
            let offset = Vec2::from([i, j]).cast();
            let pn = point2d(perm, x0 + offset);
            let dn = (pn + offset - dx).norm_l2_squared();
            min_dist_sq = min_dist_sq.min(dn);
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = Vec3::from(point);
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let dx = x - x0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                let offset = Vec3::from([i, j, k]).cast();
                let pn = point3d(perm, x0 + offset);
                let dn = (pn + offset - dx).norm_l2_squared();
                min_dist_sq = min_dist_sq.min(dn);
            }
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = Vec4::from(point);
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let dx = x - x0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for l in -1..=1 {
                    let offset = Vec4::from([i, j, k, l]).cast();
                    let pn = point4d(perm, x0 + offset);
                    let dn = (pn + offset - dx).norm_l2_squared();
                    min_dist_sq = min_dist_sq.min(dn);
                }
            }
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

#[inline]
fn point1d(perm: &PermutationTable, x0: f64) -> f64 {
    let x = unsafe { perm.hash1d((x0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE)) };
    x as f64 / PERMUTATION_TABLE_SIZE as f64
}

#[inline]
fn point2d(perm: &PermutationTable, x0: Vec2<f64>) -> Vec2<f64> {
    let x = unsafe { perm.hash2d_vec(x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE)) };
    let y = unsafe { perm.hash1d(x) };
    Vec2::from([x, y]).cast() / PERMUTATION_TABLE_SIZE as f64
}

#[inline]
fn point3d(perm: &PermutationTable, x0: Vec3<f64>) -> Vec3<f64> {
    let x = unsafe { perm.hash3d_vec(x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE)) };
    let y = unsafe { perm.hash1d(x) };
    let z = unsafe { perm.hash1d(y) };
    Vec3::from([x, y, z]).cast() / PERMUTATION_TABLE_SIZE as f64
}

#[inline]
fn point4d(perm: &PermutationTable, x0: Vec4<f64>) -> Vec4<f64> {
    let x = unsafe { perm.hash4d_vec(x0.cast().rem_euclid(PERMUTATION_TABLE_SIZE)) };
    let y = unsafe { perm.hash1d(x) };
    let z = unsafe { perm.hash1d(y) };
    let w = unsafe { perm.hash1d(z) };
    Vec4::from([x, y, z, w]).cast() / PERMUTATION_TABLE_SIZE as f64
}
