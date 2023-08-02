use libnoise::{sources, utils::Visualizer, Generator};

fn main() {
    example_chaining();
}

fn example_chaining() {
    let generator = sources::simplex(42)
        .fractal(3, 0.013, 1.0, 2.0, 0.5)
        .abs()
        .mul(2.0)
        .lambda(|x| 1.0 - x.exp() / 2.8);
    Visualizer::<2>::new([1000, 1000], generator).write_to_file("chaining.png");
}
