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

pub fn solve_second_captcha(captcha: &str) -> u32 {
    captcha
        .chars()
        .enumerate()
        .map(|(index, c)| if c ==
            captcha
                .chars()
                .cycle()
                .nth(captcha.len() / 2 + index)
                .unwrap()
        {
            c.to_digit(10).unwrap_or(0)
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

    #[test]
    fn second_captcha_all_digit_matches() {
        assert_eq!(6, solve_second_captcha("1212"));
    }

    #[test]
    fn second_captcha_no_matches() {
        assert_eq!(0, solve_second_captcha("1221"));
    }

    #[test]
    fn second_captcha_two_matches() {
        assert_eq!(4, solve_second_captcha("123425"));
    }

    #[test]
    fn second_captcha_three_matches() {
        assert_eq!(12, solve_second_captcha("123123"));
    }

    #[test]
    fn second_captcha_ones_match() {
        assert_eq!(4, solve_second_captcha("12131415"));
    }
}
