pub fn hash(list_size: usize, lengths: &[usize]) -> Vec<u32> {
    let mut list = (0..(list_size as u32)).collect::<Vec<_>>();

    let mut position = 0;

    for (skip_size, &length) in lengths.iter().enumerate() {
        let items = list.iter()
            .enumerate()
            .cycle()
            .skip(position)
            .take(length)
            .map(|(index, &item)| (index, item))
            .collect::<Vec<_>>();

        for i in 0..items.len() {
            list[items[items.len() - 1 - i].0] = items[i].1;
        }

        position += length + skip_size;
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hash() {
        assert_eq!(vec![3, 4, 2, 1, 0], hash(5, &[3, 4, 1, 5]));
    }
}
