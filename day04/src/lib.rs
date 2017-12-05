pub fn count_valid_passwords(passwords: &[String]) -> usize {
    passwords.iter().filter(|p| valid_password(p)).count()
}

pub fn count_passwords_without_anagrams(passwords: &[String]) -> usize {
    passwords.iter().filter(|p| !contains_anagrams(p)).count()
}

fn valid_password(password: &str) -> bool {
    let words = password.split_whitespace().collect::<Vec<_>>();

    let mut unique_words = words.clone();
    unique_words.sort();
    unique_words.dedup();

    words.len() == unique_words.len()
}

fn contains_anagrams(password: &str) -> bool {
    let words = password.split_whitespace().collect::<Vec<_>>();

    for (index, word_1) in words.iter().enumerate() {
        for word_2 in words[(index + 1)..].iter() {
            let mut chars_1 = word_1.chars().collect::<Vec<_>>();
            let mut chars_2 = word_2.chars().collect::<Vec<_>>();

            chars_1.sort();
            chars_2.sort();

            if chars_1 == chars_2 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_valid_password() {
        assert_eq!(1, count_valid_passwords(&["aa bb cc dd ee".to_owned()]));
    }

    #[test]
    fn single_invalid_password() {
        assert_eq!(0, count_valid_passwords(&["aa bb cc dd aa".to_owned()]));
    }

    #[test]
    fn another_valid_password() {
        assert_eq!(1, count_valid_passwords(&["aa bb cc dd aaa".to_owned()]));
    }

    #[test]
    fn single_valid_password_without_anagrams() {
        assert_eq!(
            1,
            count_passwords_without_anagrams(&["abcde fghij".to_owned()])
        );
    }

    #[test]
    fn single_invalid_password_with_anagrams() {
        assert_eq!(
            0,
            count_passwords_without_anagrams(&["abcde xyz ecdab".to_owned()])
        );
    }

    #[test]
    fn another_valid_password_without_anagrams() {
        assert_eq!(
            1,
            count_passwords_without_anagrams(&["a ab abc abd abf abj".to_owned()])
        );
    }

    #[test]
    fn long_valid_password_without_anagrams() {
        assert_eq!(
            1,
            count_passwords_without_anagrams(&["iiii oiii ooii oooi oooo".to_owned()])
        );
    }

    #[test]
    fn another_invalid_password_with_anagrams() {
        assert_eq!(
            0,
            count_passwords_without_anagrams(&["oiii ioii iioi iiio".to_owned()])
        );
    }
}
