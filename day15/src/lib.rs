const DIVISOR: u64 = 2147483647;
const GENERATOR_A_FACTOR: u64 = 16807;
const GENERATOR_B_FACTOR: u64 = 48271;
const GENERATOR_A_MULTIPLES_OF: u64 = 4;
const GENERATOR_B_MULTIPLES_OF: u64 = 8;

#[derive(Clone)]
pub struct Generator {
    factor: u64,
    previous: u64,
    multiples_of: u64,
}

impl Generator {
    pub fn new(factor: u64, previous: u64, multiples_of: u64) -> Self {
        Generator {
            factor,
            previous,
            multiples_of,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.previous = (self.previous * self.factor) % DIVISOR;

            if self.previous % self.multiples_of == 0 {
                return Some(self.previous);
            }
        }
    }
}

pub struct Judge {
    generator_a: Generator,
    generator_b: Generator,
}

impl Judge {
    pub fn new(generator_a_previous: u64, generator_b_previous: u64) -> Self {
        Judge {
            generator_a: Generator::new(
                GENERATOR_A_FACTOR,
                generator_a_previous,
                GENERATOR_A_MULTIPLES_OF,
            ),
            generator_b: Generator::new(
                GENERATOR_B_FACTOR,
                generator_b_previous,
                GENERATOR_B_MULTIPLES_OF,
            ),
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
        let mut generator = Generator::new(GENERATOR_A_FACTOR, 65, 1);

        assert_eq!(Some(1092455), generator.next());
        assert_eq!(Some(1181022009), generator.next());
        assert_eq!(Some(245556042), generator.next());
        assert_eq!(Some(1744312007), generator.next());
        assert_eq!(Some(1352636452), generator.next());
    }

    #[test]
    fn test_generator_a_with_multiples() {
        let mut generator = Generator::new(GENERATOR_A_FACTOR, 65, GENERATOR_A_MULTIPLES_OF);

        assert_eq!(Some(1352636452), generator.next());
        assert_eq!(Some(1992081072), generator.next());
        assert_eq!(Some(530830436), generator.next());
        assert_eq!(Some(1980017072), generator.next());
        assert_eq!(Some(740335192), generator.next());
    }

    #[test]
    fn test_generator_b() {
        let mut generator = Generator::new(GENERATOR_B_FACTOR, 8921, 1);

        assert_eq!(Some(430625591), generator.next());
        assert_eq!(Some(1233683848), generator.next());
        assert_eq!(Some(1431495498), generator.next());
        assert_eq!(Some(137874439), generator.next());
        assert_eq!(Some(285222916), generator.next());
    }

    #[test]
    fn test_generator_b_with_multiples() {
        let mut generator = Generator::new(GENERATOR_B_FACTOR, 8921, GENERATOR_B_MULTIPLES_OF);

        assert_eq!(Some(1233683848), generator.next());
        assert_eq!(Some(862516352), generator.next());
        assert_eq!(Some(1159784568), generator.next());
        assert_eq!(Some(1616057672), generator.next());
        assert_eq!(Some(412269392), generator.next());
    }

    #[test]
    fn test_judge_with_5_rounds() {
        assert_eq!(0, Judge::new(65, 8921).count_matches(5));
    }

    #[test]
    fn test_judge_with_5_million_rounds() {
        assert_eq!(309, Judge::new(65, 8921).count_matches(5_000_000));
    }
}
