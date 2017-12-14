use std::collections::HashMap;

pub struct Firewall {
    layers: HashMap<u32, Layer>,
}

impl Firewall {
    pub fn new() -> Self {
        Firewall { layers: HashMap::new() }
    }

    pub fn add_layer(&mut self, depth: u32, range: u32) {
        self.layers.insert(depth, Layer::new(range));
    }

    pub fn parse_layer(&mut self, s: &str) {
        let mut iter = s.split_whitespace();

        let depth = iter.next().unwrap().trim_matches(':').parse().unwrap_or(0);
        let range = iter.next().unwrap().parse().unwrap_or(0);

        self.add_layer(depth, range);
    }

    pub fn trip_severity(&self) -> u32 {
        let steps = self.layers.keys().max().cloned().unwrap_or(0);
        let mut severity = 0;

        for s in 0..(steps + 1) {
            if let Some(l) = self.layers.get(&s) {
                if l.scanner_position(s) == 0 {
                    severity += s * l.range;
                }
            }
        }

        severity
    }
}

struct Layer {
    range: u32,
}

impl Layer {
    fn new(range: u32) -> Self {
        Layer { range }
    }

    fn scanner_position(&self, t: u32) -> u32 {
        let position = t % ((self.range - 1) * 2);

        if position > self.range - 1 {
            (self.range - 1) * 2 - position
        } else {
            position
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_trip_severity() {
        let mut firewall = Firewall::new();

        firewall.add_layer(0, 3);
        firewall.add_layer(1, 2);
        firewall.add_layer(4, 4);
        firewall.add_layer(6, 4);

        assert_eq!(24, firewall.trip_severity());
    }
}