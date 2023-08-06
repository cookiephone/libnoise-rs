use super::constants::PERMUTATION_TABLE_SIZE;
use crate::utils::ptable::PermutationTable;

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
    let x = point[0];
    let y = point[1];
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let y0 = y.floor();
    let dx = x - x0;
    let dy = y - y0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in (-1..=1).map(|val| val as f64) {
        for j in (-1..=1).map(|val| val as f64) {
            let pn = point2d(perm, x0 + i, y0 + j);
            let dn = (pn[0] + i - dx).powi(2) + (pn[1] + j - dy).powi(2);
            min_dist_sq = min_dist_sq.min(dn);
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

pub(crate) fn noise3d(perm: &PermutationTable, point: [f64; 3]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let y0 = y.floor();
    let z0 = z.floor();
    let dx = x - x0;
    let dy = y - y0;
    let dz = z - z0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in (-1..=1).map(|val| val as f64) {
        for j in (-1..=1).map(|val| val as f64) {
            for k in (-1..=1).map(|val| val as f64) {
                let pn = point3d(perm, x0 + i, y0 + j, z0 + k);
                let dn =
                    (pn[0] + i - dx).powi(2) + (pn[1] + j - dy).powi(2) + (pn[2] + k - dz).powi(2);
                min_dist_sq = min_dist_sq.min(dn);
            }
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

pub(crate) fn noise4d(perm: &PermutationTable, point: [f64; 4]) -> f64 {
    let x = point[0];
    let y = point[1];
    let z = point[2];
    let w = point[3];
    // origin of hypercube in which input lies and relative input position
    let x0 = x.floor();
    let y0 = y.floor();
    let z0 = z.floor();
    let w0 = w.floor();
    let dx = x - x0;
    let dy = y - y0;
    let dz = z - z0;
    let dw = w - w0;
    // compute distance to closest neighbor
    let mut min_dist_sq = f64::INFINITY;
    for i in (-1..=1).map(|val| val as f64) {
        for j in (-1..=1).map(|val| val as f64) {
            for k in (-1..=1).map(|val| val as f64) {
                for l in (-1..=1).map(|val| val as f64) {
                    let pn = point4d(perm, x0 + i, y0 + j, z0 + k, w0 + l);
                    let dn = (pn[0] + i - dx).powi(2)
                        + (pn[1] + j - dy).powi(2)
                        + (pn[2] + k - dz).powi(2)
                        + (pn[3] + l - dw).powi(2);
                    min_dist_sq = min_dist_sq.min(dn);
                }
            }
        }
    }
    // finish up, restrict max distance to 1, and normalize
    min_dist_sq.sqrt().clamp(0.0, 1.0) * 2.0 - 1.0
}

fn point1d(perm: &PermutationTable, x0: f64) -> f64 {
    let x = unsafe { perm.hash1d((x0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE)) };
    x as f64 / PERMUTATION_TABLE_SIZE as f64
}

fn point2d(perm: &PermutationTable, x0: f64, y0: f64) -> [f64; 2] {
    let x = unsafe {
        perm.hash2d(
            (x0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (y0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
        )
    };
    let y = unsafe { perm.hash1d(x) };
    [
        x as f64 / PERMUTATION_TABLE_SIZE as f64,
        y as f64 / PERMUTATION_TABLE_SIZE as f64,
    ]
}

fn point3d(perm: &PermutationTable, x0: f64, y0: f64, z0: f64) -> [f64; 3] {
    let x = unsafe {
        perm.hash3d(
            (x0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (y0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (z0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
        )
    };
    let y = unsafe { perm.hash1d(x) };
    let z = unsafe { perm.hash1d(y) };
    [
        x as f64 / PERMUTATION_TABLE_SIZE as f64,
        y as f64 / PERMUTATION_TABLE_SIZE as f64,
        z as f64 / PERMUTATION_TABLE_SIZE as f64,
    ]
}

fn point4d(perm: &PermutationTable, x0: f64, y0: f64, z0: f64, w0: f64) -> [f64; 4] {
    let x = unsafe {
        perm.hash4d(
            (x0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (y0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (z0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
            (w0 as usize).rem_euclid(PERMUTATION_TABLE_SIZE),
        )
    };
    let y = unsafe { perm.hash1d(x) };
    let z = unsafe { perm.hash1d(y) };
    let w = unsafe { perm.hash1d(z) };
    [
        x as f64 / PERMUTATION_TABLE_SIZE as f64,
        y as f64 / PERMUTATION_TABLE_SIZE as f64,
        z as f64 / PERMUTATION_TABLE_SIZE as f64,
        w as f64 / PERMUTATION_TABLE_SIZE as f64,
    ]
}

#[test]
fn foo() {
    let n = 3000000;
    let scale = 0.03133573;
    let mut min = 0.0;
    let mut max = 0.0;
    let perm = PermutationTable::new(1, PERMUTATION_TABLE_SIZE, true);
    for x in 0..n {
        let val = noise1d(&perm, [x as f64 * scale]);
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }
    println!("min: {}", min);
    println!("max: {}", max);
}
