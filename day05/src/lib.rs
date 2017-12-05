pub fn number_of_steps(offsets: &mut [i32]) -> u32 {
    let mut index = 0;
    let mut steps = 0;

    while index >= 0 && index < offsets.len() as i32 {
        steps += 1;
        let new_index = index + offsets[index as usize];
        offsets[index as usize] += 1;
        index = new_index;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_number_of_steps() {
        assert_eq!(5, number_of_steps(&mut vec![0, 3, 0, 1, -3]));
    }
}
