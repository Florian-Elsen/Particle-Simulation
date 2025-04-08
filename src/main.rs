mod particle;
mod chunk;
mod grid;

use chunk::initialize_chunkmap;
use grid::{initialize_particles, initialise_groups};

use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

fn main() {
    let grid_size = [500, 500, 1000];
    let chunk_size = 3.0;
    let grid_const = 1.0;
    let boundary = ((0.0, 501.0), (0.0, 1001.0), (0.0, 1001.0));
    let diameter = 1.0;
    let reaction_velocity_threshold = 3.0;
    let energy = 5.0;
    let particle_weight = 1.0;
    let time_step = 0.01;
    let iterations = 100;

    // Initialize particles
    let particles = Arc::new(RwLock::new(initialize_particles(grid_size, diameter, grid_const)));

    // Apply initial velocity to particles on the z = 0.5 plane
    {
        let mut particles_locked = particles.write().unwrap();
        let mut rng = rand::rng();
        for p in particles_locked.iter_mut() {
            if p.z == 0.5 {
                p.vx = rng.random_range(-1.0..1.0);
                p.vy = rng.random_range(-1.0..1.0);
                p.vz = rng.random_range(-1.0..1.0);
            }
        }
    }

    // Initialize chunk groups
    let chunk_groups = initialise_groups([
        grid_size[0] / 3,
        grid_size[1] / 3,
        grid_size[2] / 3,
    ]);

    // Set up progress bars
    let multi_progress = Arc::new(MultiProgress::new());
    let main_pb = multi_progress.add(ProgressBar::new(iterations as u64));
    main_pb.set_style(
        ProgressStyle::default_bar()
            .template("Simulation [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );

    let inner_pb_template = ProgressStyle::default_bar()
        .template("Processing chunk groups [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("..-");

    let start_time = Instant::now();

    for iter in 0..iterations {
        main_pb.set_message(format!("Iteration {}/{}", iter + 1, iterations));

        let chunkmap = initialize_chunkmap(&particles.read().unwrap(), chunk_size);
        let inner_pb = multi_progress.add(ProgressBar::new(chunk_groups.len() as u64));
        inner_pb.set_style(inner_pb_template.clone());

        for (_group_index, chunk_group) in chunk_groups.iter().enumerate() {
            chunk_group.par_iter().for_each(|&chunk_mid| {
                let particles_arc = Arc::clone(&particles);
                let chunkmap_ref = &chunkmap;

                let mut local_updates: HashMap<usize, particle::Particle> = HashMap::new();
                let mut chunk_particles = Vec::new();
                let mut other_particles = Vec::new();

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            let neighbor_chunk = (
                                (chunk_mid.0 as i32 + dx) as u32,
                                (chunk_mid.1 as i32 + dy) as u32,
                                (chunk_mid.2 as i32 + dz) as u32,
                            );
                            if let Some(indices) = chunkmap_ref.get(&neighbor_chunk) {
                                if neighbor_chunk == chunk_mid {
                                    chunk_particles.extend(indices.iter().copied());
                                } else {
                                    other_particles.extend(indices.iter().copied());
                                }
                            }
                        }
                    }
                }

                let particles_snapshot = particles_arc.read().unwrap();
                for &i in &chunk_particles {
                    let mut pi = particles_snapshot[i].clone();
                    for &j in &other_particles {
                        let mut pj = particles_snapshot[j].clone();
                        pi.collide(
                            &mut pj,
                            diameter,
                            particle_weight,
                            particle_weight,
                            reaction_velocity_threshold,
                            energy,
                        );
                    }
                    local_updates.insert(i, pi);
                }

                for &i in &chunk_particles {
                    let mut particle = local_updates
                        .remove(&i)
                        .unwrap_or_else(|| particles_snapshot[i].clone());
                    particle.propagate(time_step);
                    particle.collide_with_boundary(boundary);
                    local_updates.insert(i, particle);
                }

                drop(particles_snapshot);
                let mut write_lock = particles_arc.write().unwrap();
                for (i, p) in local_updates {
                    write_lock[i] = p;
                }
            });

            inner_pb.inc(1);
        }

        inner_pb.finish_with_message("Chunk groups processed.");
        main_pb.inc(1);
    }

    main_pb.finish_with_message("Simulation complete!");
    println!("Total simulation time: {:.2?}", start_time.elapsed());
}
