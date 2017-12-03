struct SpiralPattern {
    level: u32,
    index: u32,
    position: (i32, i32),
}

impl Iterator for SpiralPattern {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        self.position = if self.level == 0 && self.index == 0 {
            (0, 0)
        } else if self.index == 0 {
            (self.position.0 + 1, self.position.1)
        } else if self.index / (self.level * 2) == 0 {
            (self.position.0, self.position.1 - 1)
        } else if self.index / (self.level * 2) == 1 {
            (self.position.0 - 1, self.position.1)
        } else if self.index / (self.level * 2) == 2 {
            (self.position.0, self.position.1 + 1)
        } else if self.index / (self.level * 2) == 3 {
            (self.position.0 + 1, self.position.1)
        } else {
            self.position
        };

        self.index += 1;

        if self.index >= 8 * self.level {
            self.level += 1;
            self.index = 0;
        }

        Some(self.position)
    }
}

fn spiral_pattern() -> SpiralPattern {
    SpiralPattern { level: 0, index: 0, position: (0, 0) }
}

pub fn distance(i: usize) -> u32 {
    let position = spiral_pattern().nth(i - 1).unwrap();

    (position.0.abs() + position.1.abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_for_1() {
        assert_eq!(0, distance(1));
    }

    #[test]
    fn distance_for_12() {
        assert_eq!(3, distance(12));
    }

    #[test]
    fn distance_for_23() {
        assert_eq!(2, distance(23));
    }

    #[test]
    fn distance_for_1024() {
        assert_eq!(31, distance(1024));
    }
}
