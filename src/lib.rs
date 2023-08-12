//! A simple, performant, and customizable procedural noise generation library.
//!
//! Libnoise provides utilities to generate coherent noise and customize them
//! by applying a variety of operations which modify and combine generators.
//! With a focus on customizability, the library allows users to create custom
//! generators and modifiers.
//!
//! Most immediately relevant documentation can be found in [`Source`] and
//! [`Generator`].
//!
//! # Quickstart
//!
//! To get started easily, create a source generator using one of the many
//! sources found in [`Source`], and apply adapters documented in [`Generator`].
//!
//! ```
//! use libnoise::prelude::*;
//!
//! // build a simplex noise generator seeded with 42
//! let generator = Source::simplex(42);
//!
//! // sample the generator for input point [0.2, 0.5]
//! let value = generator.sample([0.2, 0.5]);
//! ```
//!
//! Note how the dimensionality, which is internally represented as a constant
//! generic argument, is automatically inferred by sampling the generator with
//! a 2-dimensional input point.
//!
//! Naturally, we can create more interesting complex generators:
//!
//! ```
//! use libnoise::prelude::*;
//!
//! // build a generator
//! let generator = Source::simplex(42)                 // start with simplex noise
//!     .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
//!     .blend(                                         // apply blending...
//!         Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
//!         Source::worley(44).scale([0.02, 0.02]))     // ...controlled by other worley noise
//!     .lambda(|f| (f * 2.0).sin() * 0.3 + f * 0.7 );  // apply a closure to the noise
//!
//! // sample the generator for input point [0.2, 0.5]
//! let value = generator.sample([0.2, 0.5]);
//! ```
//!
//! We can also use [`NoiseBuffer`] for efficiently filling n-dimensional arrays
//! with noise, and [`Visualizer`] to get a visual representation of a given
//! generator. The above generator produces the following image, when sampled for
//! every pixel position:
//!
//! ![image](https://raw.githubusercontent.com/cookiephone/libnoise-rs/master/images/doc_image_000_f7049b4.png)
//!
//! It is common to interpret the 3rd or 4th dimension as time, allowing us to
//! produce space-time noise such as:
//!
//! ![image](https://raw.githubusercontent.com/cookiephone/libnoise-rs/master/images/doc_image_001_f7049b4.gif)

mod core;
pub mod prelude;
pub use crate::core::adapters::*;
#[cfg(feature = "dev-tools")]
pub use crate::core::devtools;
pub use crate::core::generator::*;
pub use crate::core::source::Source;
pub use crate::core::sources::*;
pub use crate::core::utils::noisebuf::NoiseBuffer;
#[cfg(feature = "image")]
pub use crate::core::utils::visualizer::Visualizer;
