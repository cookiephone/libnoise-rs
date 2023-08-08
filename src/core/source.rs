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
/// # Creating a Generator
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
    pub fn constant(value: f64) -> Constant<D> {
        Constant::new(value)
    }

    pub fn simplex(seed: u64) -> Simplex<D> {
        Simplex::new(seed)
    }

    pub fn value(seed: u64) -> Value<D> {
        Value::new(seed)
    }

    pub fn perlin(seed: u64) -> Perlin<D> {
        Perlin::new(seed)
    }

    pub fn improved_perlin(seed: u64) -> ImprovedPerlin<D> {
        ImprovedPerlin::new(seed)
    }

    pub fn worley(seed: u64) -> Worley<D> {
        Worley::new(seed)
    }

    pub fn checkerboard() -> Checkerboard<D> {
        Checkerboard::new()
    }

    pub fn custom<F: Fn([f64; D]) -> f64>(f: F) -> Custom<D, F> {
        Custom::new(f)
    }
}
