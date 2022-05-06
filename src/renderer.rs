use crate::{Dla, Particle};
use svg::node::element::Circle;
use svg::Document;

pub fn save_dla_svg(dla: Dla) {
    let mut document = Document::new().set("viewBox", (-50, -50, 100, 100));

    for particle in &dla.particles {
        document = document.add(circle(particle));
    }

    svg::save("data/image.svg", &document).unwrap();
}

fn circle(particle: &Particle) -> Circle {
    Circle::new()
        .set("fill", "black")
        .set("cx", particle.x)
        .set("cy", particle.y)
        .set("r", particle.radius)
}
