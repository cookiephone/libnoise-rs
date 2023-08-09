use crate::core::sources::{
    Checkerboard, Constant, Custom, ImprovedPerlin, Perlin, Simplex, Value, Worley,
};

/// A struct serving as entry point for building generators.
///
/// This structs only purpose is to be used as an entry point for building generators. This is done by
/// calling the functions of this struct, all of which return objects implementing [`Generator<D>`].
/// These objects are called a source, because unlike adapters, they do not require at least one other
/// generator to be created.
///
/// # Sources for creating a generator
///
/// In the following example, we create a generator using the [`Simplex<D>`] source. While it could be
/// constructed directly, we suggest using the [`simplex()`] function instead for convenience. The
/// dimensionality of the input, represented by the constant generic parameter `D`, must be known at
/// compile time. Here it is inferred due to the call to [`sample()`] with an argument of size 2:
///
/// ```
/// # use libnoise::{Source, Generator};
/// // create a 2-dimensional simplex noise generator
/// let generator = Source::simplex(42);
///
/// // use the generatore to sample the function at a specific point
/// let value = generator.sample([0.2, 0.5]);
/// ```
///
/// The dimensionality can also be specified explicitly by providing a value for the constant generic
/// parameter `D`:
///
/// ```
/// // create a 4-dimensional simplex noise generator
/// # use libnoise::Source;
/// let generator = Source::<4>::simplex(42);
/// ```
///
/// [`Generator<D>`]: crate::Generator
/// [`simplex()`]: Source::simplex
/// [`sample()`]: crate::Generator::sample
pub struct Source<const D: usize>;

impl<const D: usize> Source<D> {
    /// Create a generator which produces the supplied value for every input point.
    ///
    /// The created generator returns `value` for every input.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// // create the generator
    /// let generator = Source::constant(6.9);
    ///
    /// assert_eq!(generator.sample([0.4, 0.5]), generator.sample([0.7, 0.3]))
    /// ```
    pub fn constant(value: f64) -> Constant<D> {
        Constant::new(value)
    }

    /// Create a generator which produces n-dimensional simplex noise.
    ///
    /// The created generator returns n-dimensional simplex noise. Simplex noise is a commonly used
    /// type of gradient noise. It is computed by dividing the input space into a simplicial lattice
    /// with each point being assigned a pseudorandom n-dimensional gradient. This randomness is
    /// solely derived from the value of `seed`. The actual noise value is determined from the
    /// relative position of the input point in the simplex it resides in as well as the gradients
    /// assigned to the simplex corners.
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Simplex noise is expected to return a value in the range [-1, 1]. However, for sufficiently
    /// large inputs (which typically are unreasonable), certain computations may overflow, resulting
    /// in the generator returning NaN instead.
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::simplex(42);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn simplex(seed: u64) -> Simplex<D> {
        Simplex::new(seed)
    }

    /// Create a generator which produces n-dimensional value noise.
    ///
    /// The created generator returns n-dimensional value noise. Value noise subdivides the input
    /// space into a grid lattice and assigns each point a pseudorandom value. This randomness is
    /// solely derived from the value of `seed`. the value for the input point is determined by
    /// smoothed interpolating the values of the corners of the hypercube in which the input lies
    /// accordingly.
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Value noise is expected to return a value in the range [-1, 1].
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::value(42);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn value(seed: u64) -> Value<D> {
        Value::new(seed)
    }

    /// Create a generator which produces n-dimensional perlin noise.
    ///
    /// The created generator returns n-dimensional perlin noise. Perlin noise is a commonly used
    /// type of gradient noise. It is computed by dividing the input space into a grid lattice with
    /// each point being assigned a pseudorandom n-dimensional gradient. This randomness is solely
    /// derived from the value of `seed`. The actual noise value is determined from the relative
    /// position of the input point in the hypercube it resides in as well as the gradients assigned
    /// to the hypercube corners.
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Perlin noise is expected to return a value in the range [-1, 1].
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::perlin(42);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn perlin(seed: u64) -> Perlin<D> {
        Perlin::new(seed)
    }

    /// Create a generator which produces n-dimensional improved perlin noise.
    ///
    /// The created generator returns n-dimensional improved perlin noise. Improved perlin noise is a
    /// commonly used type of gradient noise. It is computed by dividing the input space into a grid
    /// lattice with each point being assigned a pseudorandom n-dimensional gradient. This randomness
    /// is solely derived from the value of `seed`. The actual noise value is determined from the
    /// relative position of the input point in the hypercube it resides in as well as the gradients
    /// assigned to the hypercube corners.
    ///
    /// The changes to normal perlin noise are twofold: First, the smoothing function used for
    /// interpolation is replaced by a C2-continuous function. Second, the set of possible gradients
    /// for lattice points is modified to make the noise output appear more natural.
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Improved perlin noise is expected to return a value in the range [-1, 1].
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::improved_perlin(42);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn improved_perlin(seed: u64) -> ImprovedPerlin<D> {
        ImprovedPerlin::new(seed)
    }

    /// Create a generator which produces n-dimensional worley noise.
    ///
    /// The created generator returns n-dimensional worley noise (also called cell noise, cellular
    /// noise, voronoi noise). The noise is computed by dividing the input space into a grid lattice.
    /// Each hypercube is assigned a pseudorandom point that lies within it. This randomness is solely
    /// derived from the value of `seed`. For a given input point, the noise value is determined by
    /// computing the euclidean (L2) distance to the nearest such point.
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// Worley noise is expected to return a value in the range [-1, 1].
    /// </p>
    ///
    /// <p style="background:rgba(122,186,255,0.16);padding:0.75em;">
    /// <strong>Note:</strong>
    /// This implementation employs an optimization which in rare cases causes the results to deviate
    /// slightly from the expected value. Specifically, only the own as well as directly and diagonally
    /// adjacent hypercubes are considered. This optimization reduces the time necessary to compute
    /// this noise significantly without introducing noticeable artifacts in the output.
    /// </p>
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::worley(42);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn worley(seed: u64) -> Worley<D> {
        Worley::new(seed)
    }

    /// Create a generator which produces an n-dimensional checkerboard pattern.
    ///
    /// The created generator returns n-dimensional checkerboard pattern. That is, the input space
    /// is divided into a grid lattice wherein each hypercube is assigned either -1 or 1 such that
    /// no two adjacent hypercubes are assigned the same value. The noise value is determined by
    /// returning the value assigned to the hypercube in which the input point lies.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::checkerboard();
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn checkerboard() -> Checkerboard<D> {
        Checkerboard::new()
    }

    /// Create a generator which produces n-dimensional values based on the provided closure.
    ///
    /// The created generator returns n-dimensional values by executing the provided closure `f`
    /// for the input point and producing the result. This allows usage of adapters and other
    /// library functionality without being restricted to a specific source.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use libnoise::{Source, Generator};
    /// let generator = Source::custom(|[x, y]| x % 2.0 + (1.0 - y * y) % 3.0);
    /// let value = generator.sample([0.2, 0.5]);
    /// ```
    pub fn custom<F: Fn([f64; D]) -> f64>(f: F) -> Custom<D, F> {
        Custom::new(f)
    }
}
