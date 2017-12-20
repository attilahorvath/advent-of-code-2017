use std::str::FromStr;

pub struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

#[derive(Debug)]
pub struct ParticleParseError;

fn parse_particle_component<'a, T: Iterator<Item = &'a str>>(
    iter: &mut T,
) -> Result<(i64, i64, i64), ParticleParseError> {
    if let Some(s) = iter.next() {
        let mut parts = s.trim_matches(|c: char| !c.is_numeric() && c != '-')
            .split(',');

        let x = parts.next().unwrap_or("").parse().map_err(
            |_| ParticleParseError,
        )?;

        let y = parts.next().unwrap_or("").parse().map_err(
            |_| ParticleParseError,
        )?;

        let z = parts.next().unwrap_or("").parse().map_err(
            |_| ParticleParseError,
        )?;

        Ok((x, y, z))
    } else {
        Err(ParticleParseError)
    }
}

impl FromStr for Particle {
    type Err = ParticleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split_whitespace();

        let position = parse_particle_component(&mut components)?;
        let velocity = parse_particle_component(&mut components)?;
        let acceleration = parse_particle_component(&mut components)?;

        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

impl Particle {
    pub fn new(
        position: (i64, i64, i64),
        velocity: (i64, i64, i64),
        acceleration: (i64, i64, i64),
    ) -> Self {
        Particle {
            position,
            velocity,
            acceleration,
        }
    }

    fn step(&mut self) {
        self.velocity = (
            self.velocity.0 + self.acceleration.0,
            self.velocity.1 + self.acceleration.1,
            self.velocity.2 + self.acceleration.2,
        );

        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2,
        );
    }

    fn distance_from_origin(&self) -> u64 {
        self.position.0.abs() as u64 + self.position.1.abs() as u64 + self.position.2.abs() as u64
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem { particles: Vec::new() }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn find_closest(&mut self) -> usize {
        for _ in 0..100_000 {
            self.step();
        }

        self.closest_to_origin()
    }

    fn closest_to_origin(&self) -> usize {
        self.particles
            .iter()
            .enumerate()
            .min_by_key(|&(_, p)| p.distance_from_origin())
            .unwrap()
            .0
    }

    fn step(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_closest_particle() {
        let mut particle_system = ParticleSystem::new();

        particle_system.add_particle(Particle::new((3, 0, 0), (2, 0, 0), (-1, 0, 0)));
        particle_system.add_particle(Particle::new((4, 0, 0), (0, 0, 0), (-2, 0, 0)));

        assert_eq!(0, particle_system.find_closest());
    }
}
