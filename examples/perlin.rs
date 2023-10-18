use libnoise::prelude::*;

fn main() {
    example_perlin_noise1d();
    example_perlin_noise2d();
    example_perlin_noise3d();
    example_perlin_noise4d();
}

fn example_perlin_noise1d() {
    let generator = Source::perlin(42).scale([0.013; 1]);
    Visualizer::<1>::new([100], &generator)
        .write_to_file("perlin_1d.png")
        .unwrap();
}

fn example_perlin_noise2d() {
    let generator = Source::perlin(42).scale([0.013; 2]);
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("perlin_2d.png")
        .unwrap();
}

fn example_perlin_noise3d() {
    let generator = Source::perlin(42).scale([0.013; 3]);
    Visualizer::<3>::new([200, 200, 200], &generator)
        .write_to_file("perlin_3d.png")
        .unwrap();
}

fn example_perlin_noise4d() {
    let generator = Source::perlin(42).scale([0.033; 4]);
    Visualizer::<4>::new([60, 60, 60, 60], &generator)
        .write_to_file("perlin_4d.gif")
        .unwrap();
}
