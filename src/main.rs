use svg::node::element::Circle;
use svg::Document;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct DLA {
    pub particles: Vec<Particle>,
}

impl DLA {
    pub fn new(seed_particles: Vec<Particle>) -> DLA {
        DLA {
            particles: seed_particles,
        }
    }
}

fn main() {
    let seed_particles = vec![Particle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }];

    let dla = DLA::new(seed_particles);

    save_svg(dla);
}

fn save_svg(dla: DLA) {
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
