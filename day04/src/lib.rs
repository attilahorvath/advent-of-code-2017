pub fn count_valid_passwords(passwords: &[String]) -> usize {
    passwords.iter().filter(|p| valid_password(p)).count()
}

fn valid_password(password: &str) -> bool {
    let words = password.split_whitespace().collect::<Vec<_>>();

    let mut unique_words = words.clone();
    unique_words.sort();
    unique_words.dedup();

    words.len() == unique_words.len()
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
}
