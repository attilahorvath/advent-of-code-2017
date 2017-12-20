#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Horizontal,
    Vertical,
    Turn,
    Letter(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            ' ' => Tile::Empty,
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            '+' => Tile::Turn,
            l => Tile::Letter(l),
        }
    }
}

impl Tile {
    fn is_horizontal_or_letter(&self) -> bool {
        match *self {
            Tile::Horizontal | Tile::Letter(_) => true,
            _ => false,
        }
    }

    fn is_vertical_or_letter(&self) -> bool {
        match *self {
            Tile::Vertical | Tile::Letter(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    direction: Direction,
    position: (i32, i32),
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: Vec::new(),
            direction: Direction::Down,
            position: (0, 0),
        }
    }

    pub fn add_row(&mut self, tiles: &[Tile]) {
        self.tiles.push(tiles.to_vec());
    }

    pub fn find_path(&mut self) -> (String, u32) {
        self.position.0 = self.tiles[0]
            .iter()
            .position(|i| *i == Tile::Vertical)
            .unwrap() as i32;

        let mut letters = String::new();
        let mut steps = 0;

        loop {
            self.position = self.next_step();
            steps += 1;

            match self.get_tile(self.position) {
                Tile::Empty => break,
                Tile::Horizontal => {
                    if self.direction.is_vertical() &&
                        !self.get_tile(self.next_step()).is_vertical_or_letter()
                    {
                        break;
                    }
                }
                Tile::Vertical => {
                    if self.direction.is_horizontal() &&
                        !self.get_tile(self.next_step()).is_horizontal_or_letter()
                    {
                        break;
                    }
                }
                Tile::Turn => {
                    self.direction = if self.direction.is_horizontal() {
                        if self.get_tile(self.next_in_direction(&Direction::Up))
                            .is_vertical_or_letter()
                        {
                            Direction::Up
                        } else if self.get_tile(self.next_in_direction(&Direction::Down))
                                   .is_vertical_or_letter()
                        {
                            Direction::Down
                        } else {
                            break;
                        }
                    } else {
                        if self.get_tile(self.next_in_direction(&Direction::Left))
                            .is_horizontal_or_letter()
                        {
                            Direction::Left
                        } else if self.get_tile(self.next_in_direction(&Direction::Right))
                                   .is_horizontal_or_letter()
                        {
                            Direction::Right
                        } else {
                            break;
                        }
                    };
                }
                Tile::Letter(l) => letters.push(l),
            }
        }

        (letters, steps)
    }

    fn next_step(&self) -> (i32, i32) {
        self.next_in_direction(&self.direction)
    }

    fn next_in_direction(&self, direction: &Direction) -> (i32, i32) {
        match *direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        }
    }

    fn get_tile(&self, (x, y): (i32, i32)) -> Tile {
        if x < 0 || y < 0 || y >= self.tiles.len() as i32 ||
            x >= self.tiles[y as usize].len() as i32
        {
            Tile::Empty
        } else {
            self.tiles[y as usize][x as usize].clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_letters_and_steps() {
        let mut map = Map::new();

        for row in [
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|----E|--+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
            "                ",
        ].iter()
        {
            map.add_row(&row.chars().map(|c| c.into()).collect::<Vec<_>>());
        }

        assert_eq!(("ABCDEF".to_string(), 38), map.find_path());
    }
}
