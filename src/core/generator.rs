use crate::core::adapters;
use std::marker::Sized;

/// A trait for building a coherent noise generation pipeline.
///
/// This is the main generator trait. Every noise source and every adapter must implement this trait.
/// A noise source is a generator which, for a given point in input-space, produces a value based
/// on the noise function implemented in the source. An adapter is a generator which modifies another
/// generator by transforming input and/or output data. Some adapters require further parameterization
/// or multiple generators. This allows for powerful chaining of sources and adapters to flexibly
/// generate a wide variety of coherent noise.
///
/// The constant generic `D` represents the dimensionality of the input space, and can typically be
/// inferred without explicitly specifiying it when working with generators.
pub trait Generator<const D: usize>: Sized {
    /// Samples the generator at a given `point` and returns the resulting value.
    ///
    /// The input dimension is determined by the specific generator, and the required size of `point`
    /// changes accordingly.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// // Create a generator, here simplex noise.
    /// let generator = Source::simplex(42);
    ///
    /// // Sample the generator at a given point in 2D space.
    /// let value = generator.sample([0.2, 0.5]);
    ///
    /// // Simplex noise lies between -1.0 and 1.0.
    /// assert!(-1.0 <= value && value <= 1.0);
    /// ```
    fn sample(&self, point: [f64; D]) -> f64;

    /// Create a generator which scales input points before passing them to the underlying generator.
    ///
    /// Takes a scale factor for each dimension of the input space and crates a generator which scales
    /// each input point accordingly before passing it to the underlying generator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .scale([2.0, 0.2]);                 // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// // [0.4, 0.1] is equivalent to [0.2 * 2.0, 0.5 * 0.2]
    /// assert_eq!(value, Source::simplex(42).sample([0.4, 0.1]))
    /// ```
    #[inline]
    fn scale(self, scale: [f64; D]) -> adapters::Scale<D, Self> {
        adapters::Scale::new(self, scale)
    }

    /// Create a generator which translates input points before passing them to the underlying generator.
    ///
    /// Takes a translation offset for each dimension of the input space and crates a generator which
    /// translates each input point accordingly before passing it to the underlying generator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .translate([0.3, 1.0]);             // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// // [0.5, 1.5] is equivalent to [0.2 + 0.3, 0.5 + 1.0]
    /// assert_eq!(value, Source::simplex(42).sample([0.5, 1.5]))
    /// ```
    #[inline]
    fn translate(self, translation: [f64; D]) -> adapters::Translate<D, Self> {
        adapters::Translate::new(self, translation)
    }

    /// Create a generator which negates the results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except it changes
    /// the sign of the result.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .neg();                             // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, -Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn neg(self) -> adapters::Neg<Self> {
        adapters::Neg::new(self)
    }

    /// Create a generator returning the absolute value of the results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the absolute
    /// value of underlying result is generated.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .abs();                             // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).abs())
    /// ```
    #[inline]
    fn abs(self) -> adapters::Abs<Self> {
        adapters::Abs::new(self)
    }

    /// Create a generator applying the exponential function on results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the exponential
    /// function `e^x` is applied to the result. Here, `e` represents Euler's number, and `x` is the
    /// result of the underlying generator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .exp();                             // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).exp())
    /// ```
    #[inline]
    fn exp(self) -> adapters::Exp<Self> {
        adapters::Exp::new(self)
    }

    /// Create a generator adding `offset` to results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the `offset`
    /// is added to the result.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .add(1.5);                          // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point) + 1.5)
    /// ```
    #[inline]
    fn add(self, offset: f64) -> adapters::Add<Self> {
        adapters::Add::new(self, offset)
    }

    /// Create a generator multiplying `scale` to results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// is multiplied by `scale`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .mul(1.5);                          // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point) * 1.5)
    /// ```
    #[inline]
    fn mul(self, scale: f64) -> adapters::Mul<Self> {
        adapters::Mul::new(self, scale)
    }

    /// Create a generator raising results of the underlying generator to the power of `exponent`.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// is raised to the power of `exponent`. For `powi`, the `exponent` must be an integer,
    /// specifically `i32`. Using this function is generally faster than using `powf` and should be
    /// used, if the desired exponent is an integer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .powi(2);                           // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).powi(2))
    /// ```
    #[inline]
    fn powi(self, exponent: i32) -> adapters::Pow<Self, i32> {
        adapters::Pow::new(self, exponent)
    }

    /// Create a generator raising results of the underlying generator to the power of `exponent`.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// is raised to the power of `exponent`. For `powf`, the `exponent` must be a floating point
    /// number, specifically `f64`. Using this function is generally slower than using `powi` and
    /// should only be used, if the desired exponent is not an integer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .powf(1.5);                         // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).powf(1.5))
    /// ```
    #[inline]
    fn powf(self, exponent: f64) -> adapters::Pow<Self, f64> {
        adapters::Pow::new(self, exponent)
    }

    /// Create a generator clamping results of the underlying generator to a given interval.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// is clamped to the interval [`min`, `max`]. Specifically, `max` is generated if the result is
    /// greater than `max` and `min` is generated if the result is less than `min`. If the result was
    /// NaN, it will remain NaN after clamping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .clamp(-0.5, 0.5);                  // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).clamp(-0.5, 0.5))
    /// ```
    #[inline]
    fn clamp(self, min: f64, max: f64) -> adapters::Clamp<Self> {
        adapters::Clamp::new(self, min, max)
    }

    /// Create a generator applying the supplied closure to results of the underlying generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// of the generator modified by the closure provided.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    /// let closure = |x| x * x;
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .lambda(closure);                   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, closure(Source::simplex(42).sample(point)))
    /// ```
    #[inline]
    fn lambda<L>(self, lambda: L) -> adapters::Lambda<Self, L>
    where
        L: Fn(f64) -> f64,
    {
        adapters::Lambda::new(self, lambda)
    }

    /// Create a generator adding results of the underlying generator to results of a given other
    /// generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// of the generator is added to the result of the given generator for the same input point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .sum(Source::simplex(43));          // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point) + Source::simplex(43).sample(point))
    /// ```
    #[inline]
    fn sum<G>(self, other: G) -> adapters::Sum<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Sum::new(self, other)
    }

    /// Create a generator multiplying results of the underlying generator to results of a given other
    /// generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// of the generator is multiplied with the result of the given generator for the same input point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .product(Source::simplex(43));      // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point) * Source::simplex(43).sample(point))
    /// ```
    #[inline]
    fn product<G>(self, other: G) -> adapters::Product<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Product::new(self, other)
    }

    /// Create a generator producing the minimum of results of the underlying generator and results of
    /// a given other generator.
    ///
    /// Creates a generator which is producing either the result of the underlying generator, or the
    /// result of given the generator, whichever is less.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .min(Source::simplex(43));          // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).min(Source::simplex(43).sample(point)))
    /// ```
    #[inline]
    fn min<G>(self, other: G) -> adapters::Min<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Min::new(self, other)
    }

    /// Create a generator producing the maximum of results of the underlying generator and results of
    /// a given other generator.
    ///
    /// Creates a generator which is producing either the result of the underlying generator, or the
    /// result of given the generator, whichever is greater.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .max(Source::simplex(43));          // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).max(Source::simplex(43).sample(point)))
    /// ```
    #[inline]
    fn max<G>(self, other: G) -> adapters::Max<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Max::new(self, other)
    }

    /// Create a generator raising results of the underlying generator to the power of results of a
    /// given other generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the result
    /// of the generator is raised to the power of the result of the given generator for the same
    /// input point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .power(Source::simplex(43));        // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// assert_eq!(value, Source::simplex(42).sample(point).powf(Source::simplex(43).sample(point)))
    /// ```
    #[inline]
    fn power<G>(self, other: G) -> adapters::Power<Self, G>
    where
        G: Generator<D>,
    {
        adapters::Power::new(self, other)
    }

    /// Create a generator applying fractal brownian motion on the underlying generator.
    ///
    /// Within the context of coherent noise, fractal brownian motion is a common technique for making
    /// noise appear more natural. This is done by layering versions of noise with differently scaled
    /// inputs and outputs on top of each other.
    ///
    /// The process can be described as follows: The initial amplitude is 1, while the initial frequency
    /// is the passed `frequency` parameter. For each octave, the number of which is specified by the
    /// `octaves` parameter, the following is done: The input point is scaled by the frequency argument,
    /// and the underlying generator is sampled at that point. The sample is then multiplied by the
    /// amplitude. Each iteration, the frequency and amplitude are updated by multiplying them with
    /// `lacunarity` and `persistence` respectively. If not 1, this causes exponential growth or decay
    /// of the frequency and amplitude, as octaves are increase. The result is the normalized sum of
    /// samples from each octave.
    ///
    /// Note: The initial amplitude is not a parameter because the result of the generator is normalized.
    ///
    /// Note: Typically, a desireable value for `lacunarity` is 2 while a desireable value for `persistence`
    /// lies somewhere between 0 and 1.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let octaves = 6;
    /// let frequency = 1.0;
    /// let lacunarity = 2.0;
    /// let persistence = 0.5;
    ///
    /// // build a generator using the adapter
    /// let generator = Source::simplex(42)
    ///     .fbm(octaves, frequency, lacunarity, persistence);
    ///
    /// // sample the generator
    /// let value = generator.sample(point);
    ///
    /// // compute manually for the given point to illustrate
    /// let underlying = Source::simplex(42);
    /// let mut expected = 0.0;
    /// let mut amp = 1.0;
    /// let mut freq = frequency;
    /// for _ in 0..octaves {
    ///     expected += amp * underlying.sample(point.map(|x| x * freq));
    ///     freq *= lacunarity;
    ///     amp *= persistence;
    /// }
    /// expected /= (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
    ///
    /// assert!(value - expected < f64::EPSILON);
    /// ```
    #[inline]
    fn fbm(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Fbm<Self> {
        adapters::Fbm::new(self, octaves, frequency, lacunarity, persistence)
    }

    /// Create a generator applying an [`fbm()`]-like effect on the underlying generator.
    ///
    /// This adapter is very similar to the [`fbm()`] adapter, except instead of using the output of the
    /// underlying generator directly, the absolute value, rescaled to the [-1, 1] range, is used instead.
    /// For details, see the [`fbm()`] adapter.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong>
    /// This adapter assumes that the underlying generator produces values in the [-1, 1] range. This is
    /// because the adapter has no knowledge of the theoretical bounds of the underlying generator and
    /// must therefore assume a range for the rescaling of absolute values for each octave. The generator
    /// created by this adapter will not produce correct results, if this contract is violated.
    /// </p>
    ///
    /// Note: The initial amplitude is not a parameter because the result of the generator is normalized.
    ///
    /// Note: Typically, a desireable value for `lacunarity` is 2 while a desireable value for `persistence`
    /// lies somewhere between 0 and 1.
    ///
    /// [`fbm()`]: Generator::fbm
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let octaves = 6;
    /// let frequency = 1.0;
    /// let lacunarity = 2.0;
    /// let persistence = 0.5;
    ///
    /// // build a generator using the adapter
    /// let generator = Source::simplex(42)
    ///     .billow(octaves, frequency, lacunarity, persistence);
    ///
    /// // sample the generator
    /// let value = generator.sample(point);
    ///
    /// // compute manually for the given point to illustrate
    /// let underlying = Source::simplex(42);
    /// let mut expected = 0.0;
    /// let mut amp = 1.0;
    /// let mut freq = frequency;
    /// for _ in 0..octaves {
    ///     expected += amp * (underlying.sample(point.map(|x| x * freq)).abs() * 2.0 - 1.0);
    ///     freq *= lacunarity;
    ///     amp *= persistence;
    /// }
    /// expected /= (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
    ///
    /// assert!(value - expected < f64::EPSILON);
    /// ```
    #[inline]
    fn billow(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> adapters::Billow<Self> {
        adapters::Billow::new(self, octaves, frequency, lacunarity, persistence)
    }

    /// Create a generator applying an [`fbm()`]-like effect on the underlying generator.
    ///
    /// This adapter is very similar to the [`fbm()`] adapter. A core difference is the lack of a
    /// persistence parameter, as the amplitude of an octave is determined by the actual value from
    /// the previous octave. The result for a given octave is computed as the square of 1 subtracted
    /// by the absolute value of the underlying generator, multiplied by the amplitude. The amplitude
    /// for the next octave is the previous result, dampened by the `attenuation` parameter and
    /// clamped to the [0, 1] range. The total result is the sum of results for each octave,
    /// normalized to the [-1, 1] range. For details, see the [`fbm()`] adapter.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong>
    /// This adapter assumes that the underlying generator produces values in the [-1, 1] range. This is
    /// because the adapter has no knowledge of the theoretical bounds of the underlying generator and
    /// must therefore assume a range for the rescaling of noise for which absolute values were computed.
    /// The generator created by this adapter will not produce correct results, if this contract is
    /// violated.
    /// </p>
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// The initial amplitude is not a parameter because the result of the generator is normalized.
    /// </p>
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Typically, desireable values for `lacunarity` and `attenuation` are 2.
    /// </p>
    ///
    /// [`fbm()`]: Generator::fbm
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// let octaves = 6;
    /// let frequency = 1.0;
    /// let lacunarity = 2.0;
    /// let attenuation = 2.0;
    ///
    /// // build a generator using the adapter
    /// let generator = Source::simplex(42)
    ///     .ridgedmulti(octaves, frequency, lacunarity, attenuation);
    ///
    /// // sample the generator
    /// let value = generator.sample(point);
    ///
    /// // compute manually for the given point to illustrate
    /// let underlying = Source::simplex(42);
    /// let mut expected = 0.0;
    /// let mut amp = 1.0;
    /// let mut freq = frequency;
    /// for _ in 0..octaves {
    ///     let mut tmp = amp * (1.0 - underlying.sample(point.map(|x| x * freq)).abs()).powi(2);
    ///     expected += amp;
    ///     freq *= lacunarity;
    ///     amp = (tmp / attenuation).clamp(0.0, 1.0);
    /// }
    /// expected = (expected / (0..octaves).fold(0.0, |acc, octave| {
    ///     acc + (1.0 / attenuation).powi(octave as i32)
    /// })) * 2.0 - 1.0;
    ///
    /// assert!(value - expected < f64::EPSILON);
    /// ```
    #[inline]
    fn ridgedmulti(
        self,
        octaves: u32,
        frequency: f64,
        lacunarity: f64,
        attenuation: f64,
    ) -> adapters::RidgedMulti<Self> {
        adapters::RidgedMulti::new(self, octaves, frequency, lacunarity, attenuation)
    }

    /// Create a generator blending the underlying generator with a given other generator based on the
    /// value supplied by a control-generator.
    ///
    /// This adapter takes two generators, `other` and `control`, as parameters. The generator `control`
    /// is expected to produce values in the [-1, 1] range. Based on that value, the results of the
    /// underlying generator and `other` are blended. If the value is -1, the result is equal to that of
    /// the underlying generator. If the value is 1, the result is equal to that of `other`. For other
    /// `control` values, the result is the linear interpolation between the results of the underlying
    /// generator and `other`.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong>
    /// This adapter assumes that the control generator produces values in the [-1, 1] range.
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// // build a generator using the adapter
    /// let generator = Source::simplex(42)
    ///     .blend(Source::simplex(43), Source::simplex(44));
    ///
    /// // sample the generator
    /// let value = generator.sample(point);
    ///
    /// // compute manually for the given point to illustrate
    /// let a = Source::simplex(42).sample(point);
    /// let b = Source::simplex(43).sample(point);
    /// let t = Source::simplex(44).sample(point) * 0.5 + 0.5;
    /// let expected = a + t * (b - a);
    ///
    /// assert!(value - expected < f64::EPSILON);
    /// ```
    #[inline]
    fn blend<G, GC>(self, other: G, control: GC) -> adapters::Blend<Self, G, GC>
    where
        G: Generator<D>,
        GC: Generator<D>,
    {
        adapters::Blend::new(self, other, control)
    }

    /// Create a generator selecting the result of either the underlying generator or that of a given
    /// other generator based on whether the value supplied by a control-generator lies within the
    /// provided interval.
    ///
    /// This adapter takes two generators, `other` and `control`, as well as an interval defined by
    /// `selection_min` and `selection_max`, as parameters. If the value produced by generator `control`
    /// lies within the provided interval, produce the result of the underlying generator. Otherwise,
    /// produce the result of `other`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let point = [0.2, 0.5];
    ///
    /// // build a generator using the adapter
    /// let generator = Source::simplex(42)
    ///     .select(Source::simplex(43), Source::simplex(44), -0.3, 0.1);
    ///
    /// // sample the generator
    /// let value = generator.sample(point);
    ///
    /// // compute manually for the given point to illustrate
    /// let expected = match Source::simplex(44).sample(point) {
    ///     x if -0.3 <= x && x <= 0.1 => Source::simplex(42).sample(point),
    ///     _ => Source::simplex(43).sample(point),
    /// };
    ///
    /// assert!(value - expected < f64::EPSILON);
    /// ```
    #[inline]
    fn select<G, GC>(
        self,
        other: G,
        control: GC,
        selection_min: f64,
        selection_max: f64,
    ) -> adapters::Select<Self, G, GC>
    where
        G: Generator<D>,
        GC: Generator<D>,
    {
        adapters::Select::new(self, other, control, selection_min, selection_max)
    }
}

/// A trait representing the specialization of [`Generator<D>`] for one-dimensional input spaces.
///
/// Anything implementing this trait must also implement [`Generator<1>`]. This trait exists
/// for two reasons: The first is to provide functions that either only make sense for specific
/// dimensionalities, or are either too difficult or inefficient to implement in a
/// dimension-agnostic manner. The second is to bypass certain limitations of constant generics.
pub trait Generator1D: Generator<1> {
    /// Create a generator providing the results of the underlying generator after displacing the
    /// x-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// x-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator1D};
    /// let mut point = [0.2];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_x(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[0] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<1>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

/// A trait representing the specialization of [`Generator<D>`] for two-dimensional input spaces.
///
/// Anything implementing this trait must also implement [`Generator<2>`]. This trait exists
/// for two reasons: The first is to provide functions that either only make sense for specific
/// dimensionalities, or are either too difficult or inefficient to implement in a
/// dimension-agnostic manner. The second is to bypass certain limitations of constant generics.
pub trait Generator2D: Generator<2> {
    /// Create a generator which rotates input points before passing them to the underlying generator.
    ///
    /// Takes an angle in radians for each unique pair of axes in the input space and crates a
    /// generator which rotates each input point for the provided angle on the plane spanned by each
    /// axis pair, before passing it to the underlying generator. The specific plane of rotation
    /// for each angle is as follows:
    ///
    /// | plane of rotation | corresponding angle |
    /// |-------------------|---------------------|
    /// | `xy`-plane        | `rotation[0]`       |
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator2D};
    /// let point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .rotate([0.4]);                     // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    /// ```
    #[inline]
    fn rotate(self, rotation: [f64; 1]) -> adapters::Rotate<1, Self> {
        adapters::Rotate::new(self, rotation)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// x-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// x-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator2D};
    /// let mut point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_x(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[0] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// y-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// y-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator2D};
    /// let mut point = [0.2, 0.5];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_y(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[1] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<2>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

/// A trait representing the specialization of [`Generator<D>`] for three-dimensional input spaces.
///
/// Anything implementing this trait must also implement [`Generator<3>`]. This trait exists
/// for two reasons: The first is to provide functions that either only make sense for specific
/// dimensionalities, or are either too difficult or inefficient to implement in a
/// dimension-agnostic manner. The second is to bypass certain limitations of constant generics.
pub trait Generator3D: Generator<3> {
    /// Create a generator which rotates input points before passing them to the underlying generator.
    ///
    /// Takes an angle in radians for each unique pair of axes in the input space and crates a
    /// generator which rotates each input point for the provided angle on the plane spanned by each
    /// axis pair, before passing it to the underlying generator. The specific plane of rotation
    /// for each angle is as follows:
    ///
    /// | plane of rotation | corresponding angle |
    /// |-------------------|---------------------|
    /// | `xy`-plane        | `rotation[0]`       |
    /// | `yz`-plane        | `rotation[1]`       |
    /// | `zx`-plane        | `rotation[2]`       |
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator3D};
    /// let point = [0.2, 0.5, 0.3];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .rotate([0.4, 1.5, 2.3]);           // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    /// ```
    #[inline]
    fn rotate(self, rotation: [f64; 3]) -> adapters::Rotate<3, Self> {
        adapters::Rotate::new(self, rotation)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// x-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// x-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator3D};
    /// let mut point = [0.2, 0.5, 0.3];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_x(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[0] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// y-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// y-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator3D};
    /// let mut point = [0.2, 0.5, 0.3];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_y(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[1] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// z-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// z-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator3D};
    /// let mut point = [0.2, 0.5, 0.3];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_z(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[2] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<3>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}

/// A trait representing the specialization of [`Generator<D>`] for four-dimensional input spaces.
///
/// Anything implementing this trait must also implement [`Generator<4>`]. This trait exists
/// for two reasons: The first is to provide functions that either only make sense for specific
/// dimensionalities, or are either too difficult or inefficient to implement in a
/// dimension-agnostic manner. The second is to bypass certain limitations of constant generics.
pub trait Generator4D: Generator<4> {
    /// Create a generator which rotates input points before passing them to the underlying generator.
    ///
    /// Takes an angle in radians for each unique pair of axes in the input space and crates a
    /// generator which rotates each input point for the provided angle on the plane spanned by each
    /// axis pair, before passing it to the underlying generator. The specific plane of rotation
    /// for each angle is as follows:
    ///
    /// | plane of rotation | corresponding angle |
    /// |-------------------|---------------------|
    /// | `zw`-plane        | `rotation[0]`       |
    /// | `yw`-plane        | `rotation[1]`       |
    /// | `yz`-plane        | `rotation[2]`       |
    /// | `xw`-plane        | `rotation[3]`       |
    /// | `yz`-plane        | `rotation[4]`       |
    /// | `xy`-plane        | `rotation[5]`       |
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator4D};
    /// let point = [0.2, 0.5, 0.3, 0.7];
    ///
    /// let generator = Source::simplex(42)             // build a generator
    ///     .rotate([0.4, 1.5, 2.3, 0.9, 1.7, 3.1]);    // apply the adapter
    ///
    /// let value = generator.sample(point);            // sample the generator
    /// ```
    #[inline]
    fn rotate(self, rotation: [f64; 6]) -> adapters::Rotate<6, Self> {
        adapters::Rotate::new(self, rotation)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// x-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// x-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator4D};
    /// let mut point = [0.2, 0.5, 0.3, 0.7];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_x(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[0] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_x<GA>(self, displacement_generator: GA) -> adapters::Displace<0, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// y-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// y-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator4D};
    /// let mut point = [0.2, 0.5, 0.3, 0.7];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_y(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[1] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_y<GA>(self, displacement_generator: GA) -> adapters::Displace<1, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// z-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// z-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator4D};
    /// let mut point = [0.2, 0.5, 0.3, 0.7];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_z(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[2] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_z<GA>(self, displacement_generator: GA) -> adapters::Displace<2, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }

    /// Create a generator providing the results of the underlying generator after displacing the
    /// w-coordinate by the result of the provided generator.
    ///
    /// Creates a generator which is exactly the same as the underlying generator, except the
    /// w-coordinate of the input point is first displaced by the result of `displacement_generator`
    /// for that point.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator, Generator4D};
    /// let mut point = [0.2, 0.5, 0.3, 0.7];
    ///
    /// let generator = Source::simplex(42)     // build a generator
    ///     .displace_w(Source::simplex(43));   // apply the adapter
    ///
    /// let value = generator.sample(point);    // sample the generator
    ///
    /// point[3] += Source::simplex(43).sample(point);
    /// assert_eq!(value, Source::simplex(42).sample(point))
    /// ```
    #[inline]
    fn displace_w<GA>(self, displacement_generator: GA) -> adapters::Displace<3, Self, GA>
    where
        GA: Generator<4>,
    {
        adapters::Displace::new(self, displacement_generator)
    }
}
