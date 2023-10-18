use libnoise::prelude::*;

fn main() {
    example_custom_noise1d();
    example_custom_noise2d();
    example_custom_noise3d();
    example_custom_noise4d();
}

fn example_custom_noise1d() {
    let generator = Source::custom(|[x]| (-(x * x)).exp() * 2.0 - 1.0).scale([0.003; 1]);
    Visualizer::<1>::new([100], &generator)
        .write_to_file("custom_1d.png")
        .unwrap();
}

fn example_custom_noise2d() {
    let generator =
        Source::custom(|[x, y]| (-(x * x) - (y * y)).exp() * 2.0 - 1.0).scale([0.003; 2]);
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("custom_2d.png")
        .unwrap();
}

fn example_custom_noise3d() {
    let generator = Source::custom(|[x, y, z]| (-(x * x) - (y * y) - (z * z)).exp() * 2.0 - 1.0)
        .scale([0.007; 3]);
    Visualizer::<3>::new([200, 200, 200], &generator)
        .write_to_file("custom_3d.png")
        .unwrap();
}

fn example_custom_noise4d() {
    let generator = Source::custom(|[x, y, z, w]| {
        (-(x * x) - (y * y) - (z * z) - (w * w) * 2.0 - 1.0).exp() * 2.0 - 1.0
    })
    .scale([0.021; 4]);
    Visualizer::<4>::new([60, 60, 60, 60], &generator)
        .write_to_file("custom_4d.gif")
        .unwrap();
}
