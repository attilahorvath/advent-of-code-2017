const DIVISOR: u64 = 2147483647;
const GENERATOR_A_FACTOR: u64 = 16807;
const GENERATOR_B_FACTOR: u64 = 48271;

#[derive(Clone)]
pub struct Generator {
    factor: u64,
    previous: u64,
}

impl Generator {
    pub fn new(factor: u64, previous: u64) -> Self {
        Generator { factor, previous }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = (self.previous * self.factor) % DIVISOR;

        Some(self.previous)
    }
}

pub struct Judge {
    generator_a: Generator,
    generator_b: Generator,
}

impl Judge {
    pub fn new(generator_a_previous: u64, generator_b_previous: u64) -> Self {
        Judge {
            generator_a: Generator::new(GENERATOR_A_FACTOR, generator_a_previous),
            generator_b: Generator::new(GENERATOR_B_FACTOR, generator_b_previous),
        }
    }

    pub fn count_matches(&self, rounds: usize) -> usize {
        self.generator_a
            .clone()
            .zip(self.generator_b.clone())
            .take(rounds)
            .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_a() {
        let mut generator = Generator::new(GENERATOR_A_FACTOR, 65);

        assert_eq!(Some(1092455), generator.next());
        assert_eq!(Some(1181022009), generator.next());
        assert_eq!(Some(245556042), generator.next());
        assert_eq!(Some(1744312007), generator.next());
        assert_eq!(Some(1352636452), generator.next());
    }

    #[test]
    fn test_generator_b() {
        let mut generator = Generator::new(GENERATOR_B_FACTOR, 8921);

        assert_eq!(Some(430625591), generator.next());
        assert_eq!(Some(1233683848), generator.next());
        assert_eq!(Some(1431495498), generator.next());
        assert_eq!(Some(137874439), generator.next());
        assert_eq!(Some(285222916), generator.next());
    }

    #[test]
    fn test_judge_with_5_rounds() {
        assert_eq!(1, Judge::new(65, 8921).count_matches(5));
    }

    #[test]
    fn test_judge_with_40_million_rounds() {
        assert_eq!(588, Judge::new(65, 8921).count_matches(40_000_000));
    }
}
