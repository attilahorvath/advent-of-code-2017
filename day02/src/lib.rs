pub fn calculate_checksum(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| {
            row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0)
        })
        .sum()
}

pub fn calculate_second_checksum(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| {
            for (index, i) in row.iter().enumerate() {
                for j in row.as_slice()[(index + 1)..].iter() {
                    if i % j == 0 {
                        return i / j;
                    } else if j % i == 0 {
                        return j / i;
                    }
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_row() {
        assert_eq!(8, calculate_checksum(&vec![vec![5, 1, 9, 5]]));
    }

    #[test]
    fn two_rows() {
        assert_eq!(
            12,
            calculate_checksum(&vec![vec![5, 1, 9, 5], vec![7, 5, 3]])
        );
    }

    #[test]
    fn three_rows() {
        assert_eq!(
            18,
            calculate_checksum(&vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]])
        );
    }

    #[test]
    fn second_checksum_first_row() {
        assert_eq!(4, calculate_second_checksum(&vec![vec![5, 9, 2, 8]]));
    }

    #[test]
    fn second_checksum_two_rows() {
        assert_eq!(
            7,
            calculate_second_checksum(&vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3]])
        );
    }

    #[test]
    fn second_checksum_three_rows() {
        assert_eq!(
            9,
            calculate_second_checksum(&vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]])
        );
    }
}
