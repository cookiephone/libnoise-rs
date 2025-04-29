use crate::core::generator::{Generator, Generator1D, Generator2D, Generator3D, Generator4D};

/// Error type for errors relating to the [`Spline`] adapter.
#[derive(Debug)]
pub enum SplineError {
    NotEnoughKnots(String),
}

trait SplineCoefficients {
    fn evaluate(&self, point: f64, knot_vector: &[f64], interval_idx: usize) -> f64;
}

#[derive(Debug, Clone, Copy)]
struct CubicSplineCoefficients {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl SplineCoefficients for CubicSplineCoefficients {
    fn evaluate(&self, point: f64, knot_vector: &[f64], interval_idx: usize) -> f64 {
        let t = point - knot_vector[interval_idx];
        self.a + self.b * t + self.c * t.powi(2) + self.d * t.powi(3)
    }
}

/// A trait for implementing splines used in the [`Spline`] adapter.
pub trait SplineImpl {
    fn new(knot_vector: &[f64], knots: &[f64]) -> Result<Self, SplineError>
    where
        Self: Sized,
    {
        let mut spline = Self::init(knot_vector, knots);
        spline.validate()?;
        spline.precompute_coefficients();
        Ok(spline)
    }

    fn init(knot_vector: &[f64], knots: &[f64]) -> Self;

    fn validate(&self) -> Result<(), SplineError>;

    fn precompute_coefficients(&mut self);

    fn evaluate(&self, point: f64) -> f64;
}

/// Implementation of natural cubic splines used in the [`Spline`] adapter.
#[derive(Clone, Debug)]
pub struct NaturalCubicSpline {
    knot_vector: Vec<f64>,
    knots: Vec<f64>,
    coefficients: Vec<CubicSplineCoefficients>,
}

impl SplineImpl for NaturalCubicSpline {
    fn init(knot_vector: &[f64], knots: &[f64]) -> Self {
        Self {
            knot_vector: knot_vector.into(),
            knots: knots.into(),
            coefficients: Vec::new(),
        }
    }

    fn validate(&self) -> Result<(), SplineError> {
        if self.knots.len() < 4 {
            return Err(SplineError::NotEnoughKnots(format!(
                "Cubic spline expected at least 4 knots, but got {}.",
                self.knots.len()
            )));
        }
        if self.knots.len() != self.knot_vector.len() {
            return Err(SplineError::NotEnoughKnots(
                "Knot vector and knots must be the same length, but they were not.".to_owned(),
            ));
        }
        if !self.knot_vector.is_sorted() {
            return Err(SplineError::NotEnoughKnots(
                "Knot vector must be sorted, but it was not.".to_owned(),
            ));
        }
        if self.knot_vector.iter().any(|x| !x.is_finite()) {
            return Err(SplineError::NotEnoughKnots(
                "Knot vector must contain finite values, but encountered either NaN, Inf or -Inf."
                    .to_owned(),
            ));
        }
        if self.knots.iter().any(|x| !x.is_finite()) {
            return Err(SplineError::NotEnoughKnots(
                "Knots must contain finite values, but encountered either NaN, Inf or -Inf."
                    .to_owned(),
            ));
        }
        Ok(())
    }

    fn precompute_coefficients(&mut self) {
        let n = self.knots.len();
        let mut h = vec![0.0; n - 1];
        let mut alpha = vec![0.0; n - 1];
        // compute h[i] and alpha[i]
        for (i, hi) in h.iter_mut().enumerate().take(n - 1) {
            *hi = self.knot_vector[i + 1] - self.knot_vector[i];
        }
        for i in 1..n - 1 {
            // Start at 1, end at n-2 (excluding boundaries)
            alpha[i] = (3.0 / h[i]) * (self.knots[i + 1] - self.knots[i])
                - (3.0 / h[i - 1]) * (self.knots[i] - self.knots[i - 1]);
        }
        // solve tridiagonal system for c[i]
        let mut l = vec![0.0; n];
        let mut mu = vec![0.0; n];
        let mut z = vec![0.0; n];
        let mut c = vec![0.0; n];
        l[0] = 1.0;
        mu[0] = 0.0;
        z[0] = 0.0;
        for i in 1..n - 1 {
            l[i] = 2.0 * (self.knot_vector[i + 1] - self.knot_vector[i - 1]) - h[i - 1] * mu[i - 1];
            mu[i] = h[i] / l[i];
            z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
        }
        l[n - 1] = 1.0;
        z[n - 1] = 0.0;
        c[n - 1] = 0.0;
        // Compute b[i], d[i], a[i]
        let mut b = vec![0.0; n - 1];
        let mut d = vec![0.0; n - 1];
        let a = self.knots[..n - 1].to_vec();
        for j in (0..n - 1).rev() {
            c[j] = z[j] - mu[j] * c[j + 1];
            b[j] =
                (self.knots[j + 1] - self.knots[j]) / h[j] - h[j] * (c[j + 1] + 2.0 * c[j]) / 3.0;
            d[j] = (c[j + 1] - c[j]) / (3.0 * h[j]);
        }
        // store coefficients
        for i in 0..n - 1 {
            self.coefficients.push(CubicSplineCoefficients {
                a: a[i],
                b: b[i],
                c: c[i],
                d: d[i],
            });
        }
    }

    fn evaluate(&self, point: f64) -> f64 {
        // handle the case where point is out of bounds
        if point < *self.knot_vector.first().unwrap() || point > *self.knot_vector.last().unwrap() {
            return f64::NAN;
        }
        // obtain the index of the interval containing the point
        let idx = self
            .knot_vector
            .binary_search_by(|x| x.partial_cmp(&point).unwrap())
            .unwrap_or_else(|idx| idx - 1);
        // evaluate the spline
        self.coefficients[idx].evaluate(point, &self.knot_vector, idx)
    }
}

/// A generator returning the absolute value of the results of the underlying generator.
///
/// Typically, this struct is not meant to be used directly. Instead, [`spline()`]
/// implemented by [`Generator`], should be used to create [`Spline`].
///
/// [`spline()`]: Generator::spline
#[derive(Clone, Debug)]
pub struct Spline<const D: usize, G, S: SplineImpl> {
    generator: G,
    spline: S,
}

impl<G: Generator<1>, S: SplineImpl> Generator1D for Spline<1, G, S> {}
impl<G: Generator<2>, S: SplineImpl> Generator2D for Spline<2, G, S> {}
impl<G: Generator<3>, S: SplineImpl> Generator3D for Spline<3, G, S> {}
impl<G: Generator<4>, S: SplineImpl> Generator4D for Spline<4, G, S> {}

impl<const D: usize, G, S: SplineImpl> Spline<D, G, S>
where
    G: Generator<D>,
{
    #[inline]
    pub fn new(generator: G, knot_vector: &[f64], knots: &[f64]) -> Self {
        let spline = SplineImpl::new(knot_vector, knots).unwrap();
        Self { generator, spline }
    }
}

impl<const D: usize, G, S> Generator<D> for Spline<D, G, S>
where
    G: Generator<D>,
    S: SplineImpl,
{
    #[inline]
    fn sample(&self, point: [f64; D]) -> f64 {
        self.spline.evaluate(self.generator.sample(point))
    }
}
