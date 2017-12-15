const LIST_SIZE: usize = 256;
const ROUNDS: usize = 64;
const BYTES_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
const CHUNKS: usize = 16;
const GRID_SIZE: usize = 128;

fn hash(bytes: &[u8]) -> Vec<char> {
    let mut list = (0..LIST_SIZE).map(|i| i as u8).collect::<Vec<_>>();

    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0..ROUNDS {
        for &length in bytes.iter().chain(BYTES_SUFFIX.iter()) {
            for i in 0..(length as usize / 2) {
                list.swap(
                    (position + i) % LIST_SIZE,
                    (position + (length as usize - 1 - i)) % LIST_SIZE,
                );
            }

            position += length as usize + skip_size;
            skip_size += 1;
        }
    }

    let hash = list.chunks(CHUNKS)
        .map(|c| c.iter().fold(0, |acc, &x| acc ^ x))
        .map(|b| format!("{:04b}{:04b}", b >> 4, b & 0xF))
        .collect::<String>();

    hash.chars().collect::<Vec<_>>()
}

pub struct Grid(Vec<Vec<char>>);

impl Grid {
    pub fn new(key: &str) -> Self {
        let mut grid = Vec::new();

        for i in 0..GRID_SIZE {
            grid.push(hash(format!("{}-{}", key, i).as_bytes()));
        }

        Grid(grid)
    }

    pub fn used_squares(&self) -> usize {
        self.0
            .iter()
            .map(|r| r.iter().filter(|&&c| c == '1').count())
            .sum()
    }

    pub fn regions(&self) -> usize {
        let mut regions = 0;
        let mut grid = self.0.clone();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if clear_region(&mut grid, x as i32, y as i32) {
                    regions += 1;
                }
            }
        }

        regions
    }
}

fn clear_region(grid: &mut Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x < 0 || x >= GRID_SIZE as i32 || y < 0 || y >= GRID_SIZE as i32 {
        return false;
    }

    {
        let square = grid[y as usize].get_mut(x as usize).unwrap();

        if *square == '0' {
            return false;
        }

        *square = '0';
    }

    clear_region(grid, x - 1, y);
    clear_region(grid, x + 1, y);
    clear_region(grid, x, y - 1);
    clear_region(grid, x, y + 1);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_used_squares() {
        assert_eq!(8108, Grid::new("flqrgnkx").used_squares());
    }

    #[test]
    fn count_regions() {
        assert_eq!(1242, Grid::new("flqrgnkx").regions());
    }
}
