use libnoise::{utils::Visualizer, Generator, Source};

fn main() {
    example_chaining();
}

fn example_chaining() {
    let generator = Source::simplex(42)
        .fbm(3, 0.013, 1.0, 2.0, 0.5)
        .abs()
        .mul(2.0)
        .rotate([0.5])
        .lambda(|x| 1.0 - x.exp() / 2.8)
        .displace_x(Source::simplex(1).fbm(3, 1.0, 1.0, 2.0, 0.5).mul(5.0))
        .displace_y(Source::simplex(2).fbm(3, 1.0, 1.0, 2.0, 0.5).mul(5.0));
    Visualizer::<2>::new([1000, 1000], generator).write_to_file("chaining.png");
}
