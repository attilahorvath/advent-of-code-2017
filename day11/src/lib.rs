use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug, PartialEq)]
pub struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "n" => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "se" => Ok(Direction::SE),
            "s" => Ok(Direction::S),
            "sw" => Ok(Direction::SW),
            "nw" => Ok(Direction::NW),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HexCoords {
    x: i32,
    y: i32,
    z: i32,
    max_distance: u32,
}

impl HexCoords {
    pub fn new() -> Self {
        HexCoords {
            x: 0,
            y: 0,
            z: 0,
            max_distance: 0,
        }
    }

    pub fn take_step(&mut self, step: Direction) {
        match step {
            Direction::N => {
                self.y += 1;
                self.z -= 1;
            }
            Direction::NE => {
                self.x += 1;
                self.z -= 1;
            }
            Direction::SE => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::S => {
                self.y -= 1;
                self.z += 1;
            }
            Direction::SW => {
                self.x -= 1;
                self.z += 1;
            }
            Direction::NW => {
                self.x -= 1;
                self.y += 1;
            }
        }

        let current_distance = self.distance_from_origin();

        if current_distance > self.max_distance {
            self.max_distance = current_distance;
        }
    }

    pub fn distance_from_origin(&self) -> u32 {
        [self.x, self.y, self.z]
            .iter()
            .map(|i| i.abs() as u32)
            .max()
            .unwrap()
    }

    pub fn max_distance_from_origin(&self) -> u32 {
        self.max_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn going_northeast() {
        let mut coords = HexCoords::new();

        coords.take_step(Direction::NE);
        coords.take_step(Direction::NE);
        coords.take_step(Direction::NE);

        assert_eq!(3, coords.distance_from_origin());
    }

    #[test]
    fn going_northeast_then_coming_back() {
        let mut coords = HexCoords::new();

        coords.take_step(Direction::NE);
        coords.take_step(Direction::NE);
        coords.take_step(Direction::SW);
        coords.take_step(Direction::SW);

        assert_eq!(0, coords.distance_from_origin());
    }

    #[test]
    fn going_northeast_then_south() {
        let mut coords = HexCoords::new();

        coords.take_step(Direction::NE);
        coords.take_step(Direction::NE);
        coords.take_step(Direction::S);
        coords.take_step(Direction::S);

        assert_eq!(2, coords.distance_from_origin());
    }

    #[test]
    fn going_southeast_and_southwest() {
        let mut coords = HexCoords::new();

        coords.take_step(Direction::SE);
        coords.take_step(Direction::SW);
        coords.take_step(Direction::SE);
        coords.take_step(Direction::SW);
        coords.take_step(Direction::SW);

        assert_eq!(3, coords.distance_from_origin());
    }
}
