use geode::{Dla, Particle};

fn main() {
    let seed_particles = vec![Particle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }];

    let dla = Dla::new(seed_particles);

    geode::save_dla_svg(dla);
}
