use crate::ptable::build_permutation_table;

macro_rules! grad {
    ($e:expr) => {
        GRADIENT_LUT_2D[$e]
    };
}

const SKEW_FACTOR_2D: f64 = 0.3660254037844386;
const UNSKEW_FACTOR_2D: f64 = 0.21132486540518713;

const GRADIENT_LUT_2D: [[f64; 2]; 4] = [[0.0, -1.0], [-1.0, 0.0], [0.0, 1.0], [1.0, 0.0]];
const GRADIENT_LUT_3D: [[f64; 3]; 12] = [
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
const GRADIENT_LUT_4D: [[f64; 4]; 32] = [
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
];

pub struct Simplex {
    pub seed: u64,
    pub w: usize,
    pub r2: f64,
    permutation_table: Vec<usize>,
}

impl Simplex {
    pub fn new(seed: u64, w: usize, r2: f64) -> Self {
        let permutation_table = build_permutation_table(seed, w, false);
        Self {
            seed,
            w,
            r2,
            permutation_table,
        }
    }

    pub fn noise2d(&self, x: f64, y: f64) -> f64 {
        macro_rules! hash {
            ($e:expr) => {
                self.permutation_table[($e) % self.w]
            };
        }
        // transform into lattice space and floor for cube origin
        let skew = (x + y) * SKEW_FACTOR_2D;
        let is = (x + skew).floor();
        let js = (y + skew).floor();
        // input point relative to unskewed cube (and simplex) origin in source space
        let unskew = (is + js) * UNSKEW_FACTOR_2D;
        let x0 = x - is - unskew;
        let y0 = y - js - unskew;
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
        // hashed gradient indices
        let is = is as usize;
        let js = js as usize;
        let gi0 = hash![is + hash![js]] % 4;
        let gi1 = hash![is + i1 as usize + hash![js + j1 as usize]] % 4;
        let gi2 = hash![is + 1 + hash![js + 1]] % 4;
        // contributions
        let n0 = self.contribution2d(x0, y0, &grad![gi0]);
        let n1 = self.contribution2d(x1, y1, &grad![gi1]);
        let n2 = self.contribution2d(x2, y2, &grad![gi2]);
        // combine contributions and scale to [-1, 1]
        (n0 + n1 + n2) * 99.83685446303647
    }

    fn contribution2d(&self, x: f64, y: f64, gradient: &[f64]) -> f64 {
        let t = self.r2 - x * x - y * y;
        if t <= 0.0 {
            0.0
        } else {
            t.powi(4) * (gradient[0] * x + gradient[1] * y)
        }
    }
}
