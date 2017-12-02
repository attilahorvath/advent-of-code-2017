pub fn calculate_checksum(rows: &Vec<Vec<u32>>) -> u32 {
    rows.iter()
        .map(|row| {
            row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0)
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
}
