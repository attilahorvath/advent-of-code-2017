use std::str::FromStr;

pub enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug)]
pub struct DanceMoveParseError;

impl FromStr for DanceMove {
    type Err = DanceMoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();

        match chars.next() {
            Some('s') => {
                chars
                    .collect::<String>()
                    .parse()
                    .map(|s| DanceMove::Spin(s))
                    .map_err(|_| DanceMoveParseError)
            }
            Some('x') => {
                let s = chars.collect::<String>();
                let mut parts = s.split('/');

                let a = parts.next().unwrap().parse().map_err(
                    |_| DanceMoveParseError,
                )?;
                let b = parts.next().unwrap().parse().map_err(
                    |_| DanceMoveParseError,
                )?;

                Ok(DanceMove::Exchange(a, b))
            }
            Some('p') => {
                let s = chars.collect::<String>();
                let mut parts = s.split('/');

                let a = parts.next().unwrap().parse().map_err(
                    |_| DanceMoveParseError,
                )?;
                let b = parts.next().unwrap().parse().map_err(
                    |_| DanceMoveParseError,
                )?;

                Ok(DanceMove::Partner(a, b))
            }
            _ => Err(DanceMoveParseError),
        }
    }
}

pub struct Dance {
    programs: Vec<char>,
}

impl Dance {
    pub fn new(count: u8) -> Self {
        let programs = (('a' as u8)..('a' as u8 + count))
            .map(|i| i as char)
            .collect();

        Dance { programs }
    }

    pub fn step(&mut self, dance_move: DanceMove) {
        match dance_move {
            DanceMove::Spin(size) => {
                let len = self.programs.len();

                self.programs = self.programs
                    .iter()
                    .cycle()
                    .skip(len - size)
                    .take(len)
                    .cloned()
                    .collect();
            }
            DanceMove::Exchange(a, b) => {
                self.programs.swap(a, b);
            }
            DanceMove::Partner(a, b) => {
                let a_index = self.programs.iter().position(|&c| c == a).unwrap();
                let b_index = self.programs.iter().position(|&c| c == b).unwrap();

                self.programs.swap(a_index, b_index);
            }
        }
    }

    pub fn order(&self) -> String {
        self.programs.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dance_moves() {
        let mut dance = Dance::new(5);

        dance.step(DanceMove::Spin(1));
        dance.step(DanceMove::Exchange(3, 4));
        dance.step(DanceMove::Partner('e', 'b'));

        assert_eq!("baedc".to_string(), dance.order());
    }
}
