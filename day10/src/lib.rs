const LIST_SIZE: usize = 256;
const ROUNDS: usize = 64;
const BYTES_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];
const CHUNKS: usize = 16;

pub fn hash(bytes: &[u8]) -> String {
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
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_empty_string() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", hash("".as_bytes()));
    }

    #[test]
    fn hash_test_string() {
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            hash("AoC 2017".as_bytes())
        );
    }

    #[test]
    fn hash_list() {
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", hash("1,2,3".as_bytes()));
    }

    #[test]
    fn hash_another_list() {
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", hash("1,2,4".as_bytes()));
    }
}
