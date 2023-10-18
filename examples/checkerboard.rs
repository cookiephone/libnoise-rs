use libnoise::prelude::*;

fn main() {
    example_checkerboard_noise1d();
    example_checkerboard_noise2d();
    example_checkerboard_noise3d();
    example_checkerboard_noise4d();
}

fn example_checkerboard_noise1d() {
    let generator = Source::checkerboard().scale([0.013; 1]);
    Visualizer::<1>::new([100], &generator)
        .write_to_file("checkerboard_1d.png")
        .unwrap();
}

fn example_checkerboard_noise2d() {
    let generator = Source::checkerboard().scale([0.013; 2]);
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("checkerboard_2d.png")
        .unwrap();
}

fn example_checkerboard_noise3d() {
    let generator = Source::checkerboard().scale([0.013; 3]);
    Visualizer::<3>::new([200, 200, 200], &generator)
        .write_to_file("checkerboard_3d.png")
        .unwrap();
}

fn example_checkerboard_noise4d() {
    let generator = Source::checkerboard().scale([0.033; 4]);
    Visualizer::<4>::new([60, 60, 60, 60], &generator)
        .write_to_file("checkerboard_4d.gif")
        .unwrap();
}
