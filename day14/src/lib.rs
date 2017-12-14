const LIST_SIZE: usize = 256;
const ROUNDS: usize = 64;
const BYTES_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
const CHUNKS: usize = 16;

fn hash(bytes: &[u8]) -> String {
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

    list.chunks(CHUNKS)
        .map(|c| c.iter().fold(0, |acc, &x| acc ^ x))
        .map(|b| format!("{:04b}{:04b}", b >> 4, b & 0xF))
        .collect()
}

pub fn used_squares(key: &str) -> usize {
    let mut used = 0;

    for i in 0..128 {
        used += hash(format!("{}-{}", key, i).as_bytes())
            .chars()
            .filter(|&c| c == '1')
            .count()
    }

    used
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_used_squares() {
        assert_eq!(8108, used_squares("flqrgnkx"));
    }
}
