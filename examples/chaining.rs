use libnoise::prelude::*;

fn main() {
    example_chaining();
}

fn example_chaining() {
    let generator = Source::<2>::simplex(42)
        .fbm(3, 0.013, 2.0, 0.5)
        .abs()
        .mul(2.0)
        .lambda(|x| 1.0 - x.exp() / 2.8)
        .displace_x(
            Source::worley(43)
                .scale([0.005, 0.005])
                .fbm(3, 1.0, 2.0, 0.5)
                .mul(5.0),
        )
        .rotate([0.5])
        .blend(
            Source::worley(45).scale([0.033, 0.033]),
            Source::perlin(45).scale([0.033, 0.033]).add(0.3),
        );
    Visualizer::<2>::new([1000, 1000], &generator)
        .write_to_file("chaining.png")
        .unwrap();
}
