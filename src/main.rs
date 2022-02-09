use svg::node::element::Circle;
use svg::Document;

struct Particle {
    x: f32,
    y: f32,
    radius: f32,
}

fn main() {
    let particle = Particle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    save_svg(particle);
}

fn save_svg(particle: Particle) {
    let circle = Circle::new()
        .set("fill", "black")
        .set("cx", particle.x)
        .set("cy", particle.y)
        .set("r", particle.radius);

    let document = Document::new()
        .set("viewBox", (-50, -50, 100, 100))
        .add(circle);

    svg::save("data/image.svg", &document).unwrap();
}
