use crate::particle::Particle;
use std::collections::HashMap;
use rayon::prelude::*;

pub fn assign_chunk(particle: &Particle, chunk_size: f32) -> (u32, u32, u32) {
    let chunk_x = (particle.x / chunk_size).floor() as u32;
    let chunk_y = (particle.y / chunk_size).floor() as u32;
    let chunk_z = (particle.z / chunk_size).floor() as u32;
    (chunk_x, chunk_y, chunk_z)
}


pub fn initialize_chunkmap(particles: &[Particle], chunk_size: f32) -> HashMap<(u32, u32, u32), Vec<usize>> {
    let chunkmap: HashMap<(u32, u32, u32), Vec<usize>> = particles
        .par_iter() // Parallel iterator from Rayon
        .enumerate() // Keep track of the index (i)
        .fold(
            || HashMap::new(), // Initial value for each thread
            |mut acc, (i, particle)| {
                let chunk = assign_chunk(particle, chunk_size);
                acc.entry(chunk).or_insert_with(Vec::new).push(i);
                acc
            },
        )
        .reduce(
            || HashMap::new(), // Combine function: start with an empty map
            |mut acc, other| {
                acc.extend(other); // Merge the results
                acc
            },
        );
    
    chunkmap
}