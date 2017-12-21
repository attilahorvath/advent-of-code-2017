extern crate day20;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day20::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));

    let mut particle_system = ParticleSystem::new();

    for l in file.lines() {
        let particle = l.expect("error reading file").parse().expect(
            "invlalid particle definition",
        );

        particle_system.add_particle(particle);
    }

    println!(
        "Particle eventually closest to origin: {}",
        particle_system.closest_to_origin()
    );

    particle_system.resolve_collisions();
}
