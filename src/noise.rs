use crate::ptable::build_permutation_table;

macro_rules! grad {
    ($e:expr) => {
        GRADIENT_LUT_2D[$e]
    };
}

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

struct SimplexAccelerator {
    permutation_table: Vec<usize>,
}

impl SimplexAccelerator {
    fn new(seed: u64, w: usize) -> Self {
        let permutation_table = build_permutation_table(seed, w, false);
        Self {
            permutation_table,
        }
    }
}

pub struct Simplex {
    pub seed: u64,
    pub w: usize,
    pub r2: f64,
    accelerator: SimplexAccelerator,
}

impl Simplex {
    pub fn new(seed: u64, w: usize, r2: f64) -> Self {
        let accelerator = SimplexAccelerator::new(seed, w);
        Self {
            seed,
            w,
            r2,
            accelerator,
        }
    }

    pub fn noise2d(&self, x: f64, y: f64) -> f64 {
        macro_rules! hash {
            ($e:expr) => {
                self.accelerator.permutation_table[($e) % self.w]
            };
        }

        // constants
        let skew_factor = 0.3660254037844386;
        let unskew_factor = 0.21132486540518713;

        // transform into lattice space and floor for cube origin
        let skew = (x + y) * skew_factor;
        let is = (x + skew).floor();
        let js = (y + skew).floor();

        // transform cube origin into source space
        let unskew = (is + js) * unskew_factor;
        let i = is - unskew;
        let j = js - unskew;

        // input point relative to unskewed cube (and simplex) origin
        let x0 = x - i;
        let y0 = y - j;

        // compute middle simplex vector(s) between 0-vector and 1-vector
        let mut i1 = 1.0;
        let mut j1 = 0.0;
        if x0 < y0 {
            i1 = 0.0;
            j1 = 1.0;
        }

        // imput point relative to other unskewed simplex vertices
        let x1 = x0 - i1 + unskew_factor;
        let y1 = y0 - j1 + unskew_factor;
        let x2 = x0 - 1.0 + 2.0 * unskew_factor;
        let y2 = y0 - 1.0 + 2.0 * unskew_factor;

        // hashed gradient indices
        let is = is as usize;
        let js = js as usize;
        let gi0 = hash![is + hash![js]] % 4;
        let gi1 = hash![is + i1 as usize + hash![js + j1 as usize]] % 4;
        let gi2 = hash![is + 1 + hash![js + 1]] % 4;

        // contributions
        let n0 = contribution2d(self.r2, x0, y0, &grad![gi0]);
        let n1 = contribution2d(self.r2, x1, y1, &grad![gi1]);
        let n2 = contribution2d(self.r2, x2, y2, &grad![gi2]);

        // combine contributions
        (n0 + n1 + n2) * 99.83685446303647
    }
}

fn contribution2d(r2: f64, x: f64, y: f64, gradient: &[f64]) -> f64 {
    let t = r2 - x * x - y * y;
    if t <= 0.0 {
        0.0
    } else {
        t.powi(4) * (gradient[0] * x + gradient[1] * y)
    }
}

fn build_gradient_lut(dim: usize) -> Vec<Vec<f64>> {
    let n_nonzero_choices = 2_usize.pow((dim - 1) as u32);
    let n_gradients = dim * n_nonzero_choices;
    (0..n_gradients)
        .map(|i| {
            let zero_pos = i % dim;
            let signature = i / dim;
            let mut gradient = (0..(dim - 1))
                .map(|n| (signature >> n) & 1)
                .map(|bit| if bit == 0 { -1.0 } else { 1.0 })
                .collect::<Vec<f64>>();
            gradient.shrink_to_fit();
            gradient.insert(zero_pos, 0.0);
            gradient
        })
        .collect()
}
