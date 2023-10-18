use libnoise::prelude::*;

fn main() {
    example_fbm_simplex_noise1d();
    example_fbm_simplex_noise2d();
    example_fbm_simplex_noise3d();
    example_fbm_simplex_noise4d();
}

fn example_fbm_simplex_noise1d() {
    let generator = Source::simplex(42).fbm(3, 0.013, 2.0, 0.5);
    Visualizer::<1>::new([100], &generator)
        .write_to_file("fbm_simplex_1d.png")
        .unwrap();
}

fn example_fbm_simplex_noise2d() {
    let generator = Source::simplex(42).fbm(3, 0.013, 2.0, 0.5);
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("fbm_simplex_2d.png")
        .unwrap();
}

fn example_fbm_simplex_noise3d() {
    let generator = Source::simplex(42).fbm(3, 0.013, 2.0, 0.5);
    Visualizer::<3>::new([200, 200, 200], &generator)
        .write_to_file("fbm_simplex_3d.png")
        .unwrap();
}

fn example_fbm_simplex_noise4d() {
    let generator = Source::simplex(42).fbm(3, 0.033, 2.0, 0.5);
    Visualizer::<4>::new([60, 60, 60, 60], &generator)
        .write_to_file("fbm_simplex_4d.gif")
        .unwrap();
}
