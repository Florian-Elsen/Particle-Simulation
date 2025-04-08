#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub has_reacted: bool, // Indicates whether the particle has reacted
}

impl Particle {
    pub fn new(x: f32, y: f32, z: f32, vx: f32, vy: f32, vz: f32, has_reacted: bool) -> Self {
        Particle { x, y, z, vx, vy, vz, has_reacted }
    }

    pub fn propagate(&mut self, time_step: f32) {
        self.x += self.vx * time_step;
        self.y += self.vy * time_step;
        self.z += self.vz * time_step;
    }

    pub fn collide_with_boundary(&mut self, boundary: ((f32, f32), (f32, f32), (f32, f32))) {
        let (min_x, max_x) = boundary.0;
        let (min_y, max_y) = boundary.1;
        let (min_z, max_z) = boundary.2;

        if self.x < min_x { self.x = min_x; self.vx = -self.vx; }
        if self.x > max_x { self.x = max_x; self.vx = -self.vx; }

        if self.y < min_y { self.y = min_y; self.vy = -self.vy; }
        if self.y > max_y { self.y = max_y; self.vy = -self.vy; }

        if self.z < min_z { self.z = min_z; self.vz = -self.vz; }
        if self.z > max_z { self.z = max_z; self.vz = -self.vz; }
    }

    pub fn collide(
        &mut self,
        other: &mut Particle,
        diameter: f32,
        mass_self: f32,
        mass_other: f32,
        reaction_velocity_threshold: f32,
        energy: f32 // Energy passed as argument
    ) -> Option<f32> {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if distance < diameter {
            let normal = [dx / distance, dy / distance, dz / distance];

            let relative_velocity = [
                other.vx - self.vx,
                other.vy - self.vy,
                other.vz - self.vz,
            ];

            let dot_product = normal[0] * relative_velocity[0]
                + normal[1] * relative_velocity[1]
                + normal[2] * relative_velocity[2];

            if dot_product > 0.0 {
                let impulse = 2.0 * dot_product / (mass_self + mass_other);
                self.vx += impulse * mass_other * normal[0];
                self.vy += impulse * mass_other * normal[1];
                self.vz += impulse * mass_other * normal[2];
                other.vx -= impulse * mass_self * normal[0];
                other.vy -= impulse * mass_self * normal[1];
                other.vz -= impulse * mass_self * normal[2];
            }

            let relative_speed = (relative_velocity[0].powi(2) 
                + relative_velocity[1].powi(2) 
                + relative_velocity[2].powi(2)).sqrt();

            if relative_speed >= reaction_velocity_threshold {
                let mut energy_released = 0.0;
                if !self.has_reacted && !other.has_reacted {
                    energy_released = energy; // Use the energy passed from main.rs
                    self.has_reacted = true;
                    other.has_reacted = true;
                } else if !self.has_reacted || !other.has_reacted {
                    energy_released = energy / 2.0; // Adjust if only one has reacted
                    self.has_reacted = true;
                    other.has_reacted = true;
                }

                let velocity_boost = (2.0 * energy_released / mass_self).sqrt();
                self.vx += velocity_boost;
                self.vy += velocity_boost;
                self.vz += velocity_boost;

                let velocity_boost_other = (2.0 * energy_released / mass_other).sqrt();
                other.vx += velocity_boost_other;
                other.vy += velocity_boost_other;
                other.vz += velocity_boost_other;

                return Some(energy_released);
            }
        }
        None
    }
}