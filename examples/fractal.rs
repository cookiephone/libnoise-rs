use libnoise::{
    combiners, generators,
    utils::{NoiseBuffer, Visualizer},
};

fn main() {
    example_fractal_simplex_noise1d();
    example_fractal_simplex_noise2d();
    example_fractal_simplex_noise3d();
    example_fractal_simplex_noise4d();
}

fn example_fractal_simplex_noise1d() {
    let shape = &[100];
    let generator =
        combiners::fractal::apply(generators::simplex::noise1d, 3, 0.013, 1.0, 2.0, 0.5);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("fractal_simplex_1d.png");
}

fn example_fractal_simplex_noise2d() {
    let shape = &[1000, 1000];
    let generator =
        combiners::fractal::apply(generators::simplex::noise2d, 3, 0.013, 1.0, 2.0, 0.5);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("fractal_simplex_2d.png");
}

fn example_fractal_simplex_noise3d() {
    let shape = &[200, 200, 200];
    let generator =
        combiners::fractal::apply(generators::simplex::noise3d, 3, 0.013, 1.0, 2.0, 0.5);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("fractal_simplex_3d.png");
}

fn example_fractal_simplex_noise4d() {
    let shape = &[60, 60, 60, 60];
    let generator =
        combiners::fractal::apply(generators::simplex::noise4d, 3, 0.033, 1.0, 2.0, 0.5);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("fractal_simplex_4d.gif");
}
