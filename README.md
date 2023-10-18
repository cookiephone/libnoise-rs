# libnoise

![format](https://github.com/cookiephone/libnoise-rs/actions/workflows/format.yaml/badge.svg)
![lint](https://github.com/cookiephone/libnoise-rs/actions/workflows/lint.yaml/badge.svg)
![test](https://github.com/cookiephone/libnoise-rs/actions/workflows/test.yaml/badge.svg)
[![codecov](https://coveralls.io/repos/github/cookiephone/libnoise-rs/badge.svg?branch=master)](https://coveralls.io/github/cookiephone/libnoise-rs?branch=master)
[![docs.rs](https://img.shields.io/docsrs/libnoise)](https://docs.rs/libnoise)
[![Crates.io](https://img.shields.io/crates/v/libnoise)](https://crates.io/crates/libnoise)

A simple, performant, and customizable procedural noise generation library
inspired by [libnoise for C++](https://libnoise.sourceforge.net/) featuring:

-   Easy coherent noise generation through sources provided via `Source`
-   Modular generator creation by chaining adapters to modify and combine
    generator inputs and outputs, and the ability to flexibly create
    custom generators and adapters, all through the `Generator` trait
-   Efficient and cache friendly sampling of generators through
    `NoiseBuffer` and much of the generator complexity resolving at
    compile time
-   Easy visualization of generator outputs for debugging through
    `Visualizer`

Libnoise provides utilities to generate coherent noise and customize them
by applying a variety of operations which modify and combine generators.
With a focus on customizability, the library allows users to create custom
generators and modifiers.

Most immediately relevant [documentation](https://docs.rs/libnoise) can be found in
[`Source`](https://docs.rs/libnoise/latest/libnoise/struct.Source.html) and
[`Generator`](https://docs.rs/libnoise/latest/libnoise/trait.Generator.html) docs.

## Usage

First, add the dependency to your project by editing your `Cargo.toml`:
```toml
[dependencies]
libnoise = "0.1"
```

To get started easily, create a source generator using one of the many
sources found in [`Source`](https://docs.rs/libnoise/latest/libnoise/struct.Source.html),
and apply adapters documented in [`Generator`](https://docs.rs/libnoise/latest/libnoise/trait.Generator.html).
For a more detailed introduction, see the [quickstart guide](https://docs.rs/libnoise/latest/libnoise/).


```rs
use libnoise::prelude::*;

// build a simplex noise generator seeded with 42
let generator = Source::simplex(42);

// sample the generator for input point [0.2, 0.5]
let value = generator.sample([0.2, 0.5]);
```

Note how the dimensionality, which is internally represented as a constant
generic argument, is automatically inferred by sampling the generator with
a 2-dimensional input point.

Naturally, we can create more interesting complex generators:

```rs
use libnoise::prelude::*;

// build a generator
let generator = Source::simplex(42)                 // start with simplex noise
    .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
    .blend(                                         // apply blending...
        Source::worley(43).scale([0.05, 0.05]),     // ...with scaled worley noise
        Source::worley(44).scale([0.02, 0.02]))     // ...controlled by other worley noise
    .lambda(|f| (f * 2.0).sin() * 0.3 + f * 0.7 );  // apply a closure to the noise

// sample the generator for input point [0.2, 0.5]
let value = generator.sample([0.2, 0.5]);
```

We can also use [`NoiseBuffer`](https://docs.rs/libnoise/latest/libnoise/struct.NoiseBuffer.html) for efficiently filling n-dimensional arrays
with noise, and [`Visualizer`](https://docs.rs/libnoise/latest/libnoise/struct.Visualizer.html) to get a visual representation of a given
generator. The above generator produces the following image, when sampled for
every pixel position:

![image](https://raw.githubusercontent.com/cookiephone/libnoise-rs/master/images/doc_image_000_f7049b4.png)

It is common to interpret the 3rd or 4th dimension as time, allowing us to
produce space-time noise such as:

![image](https://raw.githubusercontent.com/cookiephone/libnoise-rs/master/images/doc_image_001_f7049b4.gif)

## Contributing

Contributors and feature suggestions are welcome!

Should you want to contact me directly, it is best via discord (username: oogie).

## License

Libnoise is distributed under the terms of the MIT license.

See [LICENSE-MIT](LICENSE.md) for details.
