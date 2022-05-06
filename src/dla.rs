pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct Dla {
    pub particles: Vec<Particle>,
}

impl Dla {
    pub fn new(seed_particles: Vec<Particle>) -> Dla {
        Dla {
            particles: seed_particles,
        }
    }
}
