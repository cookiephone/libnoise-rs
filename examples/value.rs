use libnoise::{
    generators, transforms,
    utils::{NoiseBuffer, Visualizer},
};

fn main() {
    example_value_noise1d();
    example_value_noise2d();
    example_value_noise3d();
    example_value_noise4d();
}

fn example_value_noise1d() {
    let shape = &[100];
    let generator = transforms::scale::apply(generators::value::noise1d, 0.013);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("value_1d.png");
}

fn example_value_noise2d() {
    let shape = &[1000, 1000];
    let generator = transforms::scale::apply(generators::value::noise2d, 0.013);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("value_2d.png");
}

fn example_value_noise3d() {
    let shape = &[200, 200, 200];
    let generator = transforms::scale::apply(generators::value::noise3d, 0.013);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("value_3d.png");
}

fn example_value_noise4d() {
    let shape = &[40, 40, 40, 40];
    let generator = transforms::scale::apply(generators::value::noise4d, 0.013);
    let noisebuf = NoiseBuffer::new(shape, generator, 42);
    Visualizer::from(noisebuf).write_to_file("value_4d.gif");
}
