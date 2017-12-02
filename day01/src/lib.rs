pub fn solve_captcha(captcha: &str) -> u32 {
    captcha
        .chars()
        .cycle()
        .take(captcha.len() + 1)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|a| if a.len() == 2 && a[0] == a[1] {
            a[0].to_digit(10).unwrap_or(0)
        } else {
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_matches() {
        assert_eq!(3, solve_captcha("1122"));
    }

    #[test]
    fn all_digit_matches() {
        assert_eq!(4, solve_captcha("1111"));
    }

    #[test]
    fn no_matches() {
        assert_eq!(0, solve_captcha("1234"));
    }

    #[test]
    fn last_digit_matches() {
        assert_eq!(9, solve_captcha("91212129"));
    }
}
