use crate::particle::Particle;


pub fn initialize_particles(grid_size: [u32; 3], particle_diameter: f32, grid_const: f32) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let offset: f32 = particle_diameter / 2.0;

    for i in 0..grid_size[0] {
        for j in 0..grid_size[1] {
            for k in 0..grid_size[2] {
                let x = i as f32 * grid_const + offset;
                let y = j as f32 * grid_const + offset;
                let z = k as f32 * grid_const + offset;

                let particle = Particle::new(x, y, z, 0.0, 0.0, 0.0, false);
                particles.push(particle);
            }
        }
    }

    particles
}


pub fn initialise_groups(last_chunk: [u32; 3]) -> Vec<Vec<(u32, u32, u32)>> {
    let mut chunk_groups: Vec<Vec<(u32, u32, u32)>> = vec![Vec::new(); 27];

    for i in 0..=last_chunk[0] {
        for j in 0..=last_chunk[1] {
            for k in 0..=last_chunk[2] {
                let index = (i % 3) * 9 + (j % 3) * 3 + (k % 3);
                chunk_groups[index as usize].push((i, j, k));
            }
        }
    }

    chunk_groups
}
