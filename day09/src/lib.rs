use std::ops::AddAssign;

#[derive(Debug, PartialEq)]
pub struct StreamData {
    pub score: u32,
    pub garbage: u32,
}

impl StreamData {
    fn new(score: u32, garbage: u32) -> Self {
        StreamData { score, garbage }
    }
}

impl AddAssign for StreamData {
    fn add_assign(&mut self, other: Self) {
        self.score += other.score;
        self.garbage += other.garbage;
    }
}

pub fn parse_stream(stream: &str) -> StreamData {
    parse_group(&mut stream.chars(), 0)
}

fn parse_group(stream: &mut Iterator<Item = char>, base_score: u32) -> StreamData {
    let mut stream_data = StreamData::new(base_score, 0);

    while let Some(c) = stream.next() {
        match c {
            '{' => stream_data += parse_group(stream, base_score + 1),
            '}' => break,
            '<' => stream_data += parse_garbage(stream),
            '!' => {
                stream.next();
            }
            _ => (),
        }
    }

    stream_data
}

fn parse_garbage(stream: &mut Iterator<Item = char>) -> StreamData {
    let mut stream_data = StreamData::new(0, 0);

    while let Some(c) = stream.next() {
        match c {
            '>' => break,
            '!' => {
                stream.next();
            }
            _ => stream_data.garbage += 1,
        }
    }

    stream_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_group() {
        assert_eq!(StreamData::new(1, 0), parse_stream("{}"));
    }

    #[test]
    fn parse_nested_groups() {
        assert_eq!(StreamData::new(6, 0), parse_stream("{{{}}}"));
    }

    #[test]
    fn parse_two_nested_groups() {
        assert_eq!(StreamData::new(5, 0), parse_stream("{{},{}}"));
    }

    #[test]
    fn parse_complex_groups() {
        assert_eq!(StreamData::new(16, 0), parse_stream("{{{},{},{{}}}}"));
    }

    #[test]
    fn parse_garbage() {
        assert_eq!(StreamData::new(1, 4), parse_stream("{<a>,<a>,<a>,<a>}"));
    }

    #[test]
    fn parse_misleading_garbage() {
        assert_eq!(StreamData::new(1, 10), parse_stream("{<{},{},{{}}>}"));
    }

    #[test]
    fn parse_garbage_groups() {
        assert_eq!(
            StreamData::new(9, 8),
            parse_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}")
        );
    }

    #[test]
    fn parse_canceled_groups() {
        assert_eq!(
            StreamData::new(9, 0),
            parse_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}")
        );
    }

    #[test]
    fn parse_canceled_garbage() {
        assert_eq!(
            StreamData::new(3, 17),
            parse_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}")
        );
    }
}
