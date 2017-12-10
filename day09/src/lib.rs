pub fn parse_score(stream: &str) -> u32 {
    parse_group(&mut stream.chars(), 0)
}

fn parse_group(stream: &mut Iterator<Item = char>, base_score: u32) -> u32 {
    let mut score = base_score;

    while let Some(c) = stream.next() {
        match c {
            '{' => score += parse_group(stream, base_score + 1),
            '}' => return score,
            '<' => parse_garbage(stream),
            '!' => { stream.next(); },
            _ => (),
        }
    }

    score
}

fn parse_garbage(stream: &mut Iterator<Item = char>) {
    while let Some(c) = stream.next() {
        match c {
            '>' => return,
            '!' => { stream.next(); },
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_group() {
        assert_eq!(1, parse_score("{}"));
    }

    #[test]
    fn parse_nested_groups() {
        assert_eq!(6, parse_score("{{{}}}"));
    }

    #[test]
    fn parse_two_nested_groups() {
        assert_eq!(5, parse_score("{{},{}}"));
    }

    #[test]
    fn parse_complex_groups() {
        assert_eq!(16, parse_score("{{{},{},{{}}}}"));
    }

    #[test]
    fn parse_garbage() {
        assert_eq!(1, parse_score("{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn parse_misleading_garbage() {
        assert_eq!(1, parse_score("{<{},{},{{}}>}"));
    }

    #[test]
    fn parse_garbage_groups() {
        assert_eq!(9, parse_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    }

    #[test]
    fn parse_canceled_groups() {
        assert_eq!(9, parse_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    }

    #[test]
    fn parse_canceled_garbage() {
        assert_eq!(3, parse_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}
