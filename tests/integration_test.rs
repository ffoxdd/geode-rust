use geode::{Dla, Particle};

#[test]
fn it_builds_a_dla() {
    let seed_particles = vec![Particle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }];

    let dla = Dla::new(seed_particles);

    assert_eq!(dla.particles.len(), 1);
}
