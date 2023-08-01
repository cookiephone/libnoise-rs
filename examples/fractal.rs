use libnoise::{combiners, sources, utils::Visualizer};

fn main() {
    example_fractal_simplex_noise1d();
    example_fractal_simplex_noise2d();
    example_fractal_simplex_noise3d();
    example_fractal_simplex_noise4d();
}

fn example_fractal_simplex_noise1d() {
    let generator = sources::Simplex::new(42);
    let generator = combiners::Fractal::new(generator, 3, 0.013, 1.0, 2.0, 0.5);
    Visualizer::<1>::new([100], generator).write_to_file("fractal_simplex_1d.png");
}

fn example_fractal_simplex_noise2d() {
    let generator = sources::Simplex::new(42);
    let generator = combiners::Fractal::new(generator, 3, 0.013, 1.0, 2.0, 0.5);
    Visualizer::<2>::new([1000, 1000], generator).write_to_file("fractal_simplex_2d.png");
}

fn example_fractal_simplex_noise3d() {
    let generator = sources::Simplex::new(42);
    let generator = combiners::Fractal::new(generator, 3, 0.013, 1.0, 2.0, 0.5);
    Visualizer::<3>::new([200, 200, 200], generator).write_to_file("fractal_simplex_3d.png");
}

fn example_fractal_simplex_noise4d() {
    let generator = sources::Simplex::new(42);
    let generator = combiners::Fractal::new(generator, 3, 0.033, 1.0, 2.0, 0.5);
    Visualizer::<4>::new([60, 60, 60, 60], generator).write_to_file("fractal_simplex_4d.gif");
}
