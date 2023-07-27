pub(crate) const R_SQUARED: f64 = 0.5;

pub(crate) const PERMUTATION_TABLE_SIZE: usize = 256;

pub(crate) const SKEW_FACTOR_2D: f64 = 0.3660254037844386;
pub(crate) const UNSKEW_FACTOR_2D: f64 = 0.21132486540518713;

pub(crate) const GRADIENT_LUT_2D_SIZE: usize = 4;

pub(crate) const GRADIENT_LUT_2D: [[f64; 2]; 4] =
    [[0.0, -1.0], [-1.0, 0.0], [0.0, 1.0], [1.0, 0.0]];

/*pub(crate) const GRADIENT_LUT_3D: [[f64; 3]; 12] = [
    [0.0, -1.0, -1.0],
    [-1.0, 0.0, -1.0],
    [-1.0, -1.0, 0.0],
    [0.0, 1.0, -1.0],
    [1.0, 0.0, -1.0],
    [1.0, -1.0, 0.0],
    [0.0, -1.0, 1.0],
    [-1.0, 0.0, 1.0],
    [-1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
];
pub(crate) const GRADIENT_LUT_4D: [[f64; 4]; 32] = [
    [0.0, -1.0, -1.0, -1.0],
    [-1.0, 0.0, -1.0, -1.0],
    [-1.0, -1.0, 0.0, -1.0],
    [-1.0, -1.0, -1.0, 0.0],
    [0.0, 1.0, -1.0, -1.0],
    [1.0, 0.0, -1.0, -1.0],
    [1.0, -1.0, 0.0, -1.0],
    [1.0, -1.0, -1.0, 0.0],
    [0.0, -1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, -1.0],
    [-1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, -1.0, 0.0],
    [0.0, 1.0, 1.0, -1.0],
    [1.0, 0.0, 1.0, -1.0],
    [1.0, 1.0, 0.0, -1.0],
    [1.0, 1.0, -1.0, 0.0],
    [0.0, -1.0, -1.0, 1.0],
    [-1.0, 0.0, -1.0, 1.0],
    [-1.0, -1.0, 0.0, 1.0],
    [-1.0, -1.0, 1.0, 0.0],
    [0.0, 1.0, -1.0, 1.0],
    [1.0, 0.0, -1.0, 1.0],
    [1.0, -1.0, 0.0, 1.0],
    [1.0, -1.0, 1.0, 0.0],
    [0.0, -1.0, 1.0, 1.0],
    [-1.0, 0.0, 1.0, 1.0],
    [-1.0, 1.0, 0.0, 1.0],
    [-1.0, 1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 0.0],
];*/
