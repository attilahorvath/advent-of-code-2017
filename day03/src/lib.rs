use std::collections::HashMap;

struct SpiralPattern {
    level: u32,
    index: u32,
    position: (i32, i32),
}

struct SpiralPatternWithValues {
    pattern: SpiralPattern,
    values: HashMap<(i32, i32), u32>,
}

impl SpiralPattern {
    fn with_values(self) -> SpiralPatternWithValues {
        SpiralPatternWithValues {
            pattern: self,
            values: HashMap::new(),
        }
    }
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

impl Iterator for SpiralPatternWithValues {
    type Item = u32;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn next(&mut self) -> Option<u32> {
        let position = self.pattern.next().unwrap();

        let mut value = self.values.get(&(position.0 - 1, position.1 - 1)).unwrap_or(&0)
            + self.values.get(&(position.0, position.1 - 1)).unwrap_or(&0)
            + self.values.get(&(position.0 + 1, position.1 - 1)).unwrap_or(&0)
            + self.values.get(&(position.0 - 1, position.1)).unwrap_or(&0)
            + self.values.get(&(position.0 + 1, position.1)).unwrap_or(&0)
            + self.values.get(&(position.0 - 1, position.1 + 1)).unwrap_or(&0)
            + self.values.get(&(position.0, position.1 + 1)).unwrap_or(&0)
            + self.values.get(&(position.0 + 1, position.1 + 1)).unwrap_or(&0);

        if position.0 == 0 && position.1 == 0 {
            value = 1;
        }

        self.values.insert(position, value);

        Some(value)
    }
}

fn spiral_pattern() -> SpiralPattern {
    SpiralPattern {
        level: 0,
        index: 0,
        position: (0, 0),
    }
}

pub fn distance(i: u32) -> u32 {
    let position = spiral_pattern().nth(i as usize - 1).unwrap();

    (position.0.abs() + position.1.abs()) as u32
}

pub fn value_greater_than(v: u32) -> u32 {
    spiral_pattern().with_values().find(|&i| i > v).unwrap()
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

    #[test]
    fn greater_than_1() {
        assert_eq!(2, value_greater_than(1));
    }

    #[test]
    fn greater_than_5() {
        assert_eq!(10, value_greater_than(5));
    }

    #[test]
    fn greater_than_130() {
        assert_eq!(133, value_greater_than(130));
    }

    #[test]
    fn greater_than_780() {
        assert_eq!(806, value_greater_than(780));
    }
}
