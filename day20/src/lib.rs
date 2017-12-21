use std::str::FromStr;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
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

    fn position_at(&self, time: i64) -> (i64, i64, i64) {
        (
            self.position.0 + self.velocity.0 * time + time * (time + 1) / 2 * self.acceleration.0,
            self.position.1 + self.velocity.1 * time + time * (time + 1) / 2 * self.acceleration.1,
            self.position.2 + self.velocity.2 * time + time * (time + 1) / 2 * self.acceleration.2,
        )
    }

    fn collides_at(&self, other: &Particle, index: usize) -> Option<i64> {
        let dp = (
            self.position.0 - other.position.0,
            self.position.1 - other.position.1,
            self.position.2 - other.position.2,
        );

        let dv = (
            self.velocity.0 - other.velocity.0,
            self.velocity.1 - other.velocity.1,
            self.velocity.2 - other.velocity.2,
        );

        let da = (
            self.acceleration.0 - other.acceleration.0,
            self.acceleration.1 - other.acceleration.1,
            self.acceleration.2 - other.acceleration.2,
        );

        let ts = if index == 2 {
            solve(da.2 as f64 / 2.0, dv.2 as f64 + da.2 as f64 / 2.0, dp.2 as f64)
        } else {
            solve(da.0 as f64 / 2.0, dv.0 as f64 + da.0 as f64 / 2.0, dp.0 as f64)
        };
        // let y = solve(da.1 / 2, dv.1 + da.1 / 2, dp.1);
        // let z = solve(da.2 / 2, dv.2 + da.2 / 2, dp.2);

        if ts.is_none() {
            return None;
        }

        let ts = ts.unwrap();

        if self.position_at(ts.0) == other.position_at(ts.0) {
            return Some(ts.0);
        } else if self.position_at(ts.1) == other.position_at(ts.1) {
            return Some(ts.1);
        }

        // a/2 * t^2 + (v + a/2) * t + p
        None
    }
}

fn solve(a: f64, b: f64, c: f64) -> Option<(i64, i64)> {
    if a == 0.0 {
        if b == 0.0 {
            return Some((c as i64, c as i64));
        }

        return Some(((-c / b) as i64, (-c / b) as i64));
    }

    let d = b.powf(2.0) - 4.0 * a * c;

    if d < 0.0 {
        return None;
    }

    let sqrt_d = d.sqrt();

    // println!("{}", sqrt_d);

    Some((
        ((-b + sqrt_d) / (2.0 * a)) as i64,
        ((-b - sqrt_d) / (2.0 * a)) as i64,
    ))
}

fn distance_from_origin((x, y, z): (i64, i64, i64)) -> u64 {
    x.abs() as u64 + y.abs() as u64 + z.abs() as u64
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

    pub fn closest_to_origin(&self) -> usize {
        self.particles
            .iter()
            .enumerate()
            .min_by_key(|&(_, p)| distance_from_origin(p.position_at(10_000)))
            .unwrap()
            .0
    }

    pub fn resolve_collisions(&self) {
        let mut collisions = BTreeMap::new();

        let mut c1 = Vec::new();
        let mut c2 = Vec::new();

        for (index, particle) in self.particles.iter().enumerate() {
            for (other_index, other) in self.particles[(index + 1)..].iter().enumerate() {
                if let Some(t) = particle.collides_at(other, 2) {
                    let c = collisions.entry(t).or_insert(vec![]);
                    (*c).push((index, other_index));
                    c1.push((index, other_index, t));
                }
            }
        }

        for (index, particle) in self.particles.iter().enumerate() {
            for (other_index, other) in self.particles[(index + 1)..].iter().enumerate() {
                if let Some(t) = particle.collides_at(other, 1) {
                    let c = collisions.entry(t).or_insert(vec![]);
                    (*c).push((index, other_index));
                    c2.push((index, other_index, t));
                }
            }
        }

        println!("{}", c1.len());
        println!("{}", c2.len());

        for i1 in c1.iter() {
            if !c2.contains(&i1) {
                println!("{:?}", i1);
                println!("{:?}", self.particles[i1.0]);
                println!("{:?}", self.particles[i1.1]);
            }
        }

        // println!("{}", collisions.iter().map(|(k, v)| v.len()).sum::<usize>());

        let mut removed = HashMap::new();

        for (t, c) in collisions.iter() {
            for &(a, b) in c {
                if (!removed.contains_key(&a) || *removed.get(&a).unwrap() < t) && (!removed.contains_key(&b) || *removed.get(&b).unwrap() < t) {
                    removed.insert(a, t);
                    removed.insert(b, t);
                }
            }
        }

        println!("{:?}", removed.len());
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

        assert_eq!(0, particle_system.closest_to_origin());
    }
}
