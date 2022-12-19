use rton_tl::generator::Generator;

fn main() {
    let generator = Generator::new("./lib/rton-tl/scheme", "./lib/rton-tl/generated");
    generator.generate();
}
