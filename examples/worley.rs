use libnoise::prelude::*;

fn main() {
    example_worley_noise1d();
    example_worley_noise2d();
    example_worley_noise3d();
    example_worley_noise4d();
}

fn example_worley_noise1d() {
    let generator = Source::worley(42).scale([0.013; 1]);
    Visualizer::<1>::new([100], &generator)
        .write_to_file("worley_1d.png")
        .unwrap();
}

fn example_worley_noise2d() {
    let generator = Source::worley(42).scale([0.013; 2]);
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("worley_2d.png")
        .unwrap();
}

fn example_worley_noise3d() {
    let generator = Source::worley(42).scale([0.013; 3]);
    Visualizer::<3>::new([200, 200, 200], &generator)
        .write_to_file("worley_3d.png")
        .unwrap();
}

fn example_worley_noise4d() {
    let generator = Source::worley(42).scale([0.033; 4]);
    Visualizer::<4>::new([60, 60, 60, 60], &generator)
        .write_to_file("worley_4d.gif")
        .unwrap();
}
