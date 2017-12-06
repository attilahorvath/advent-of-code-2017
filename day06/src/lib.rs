use std::collections::HashSet;

pub fn max_steps(blocks: &mut [u32]) -> u32 {
    let mut seen = HashSet::new();
    let mut steps = 0;
    let len = blocks.len();

    while !seen.contains(&blocks.to_vec()) {
        seen.insert(blocks.to_vec());
        steps += 1;

        let (index, &count) = blocks
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, i)| i)
            .expect("no blocks given");

        blocks[index] = 0;

        for i in 1..(count + 1) {
            blocks[(index + i as usize) % len] += 1;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_max_steps() {
        let mut blocks = vec![0, 2, 7, 0];
        assert_eq!(5, max_steps(&mut blocks));
        assert_eq!(vec![2, 4, 1, 2], blocks);
    }
}
