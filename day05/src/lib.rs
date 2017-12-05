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

pub fn number_of_steps_with_decrease(offsets: &mut [i32]) -> u32 {
    let mut index = 0;
    let mut steps = 0;

    while index >= 0 && index < offsets.len() as i32 {
        steps += 1;

        let new_index = index + offsets[index as usize];
        let offset = &mut offsets[index as usize];
        *offset += if *offset >= 3 { -1 } else { 1 };

        index = new_index;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_number_of_steps() {
        let mut offsets = vec![0, 3, 0, 1, -3];
        assert_eq!(5, number_of_steps(&mut offsets));
        assert_eq!(vec![2, 5, 0, 1, -2], offsets);
    }

    #[test]
    fn count_number_of_steps_with_decrease() {
        let mut offsets = vec![0, 3, 0, 1, -3];
        assert_eq!(10, number_of_steps_with_decrease(&mut offsets));
        assert_eq!(vec![2, 3, 2, 3, -1], offsets);
    }
}
