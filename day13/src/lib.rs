pub struct Firewall {
    layers: Vec<Option<Layer>>,
}

impl Firewall {
    pub fn new() -> Self {
        Firewall { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, depth: usize, range: u32) {
        while self.layers.len() < depth {
            self.layers.push(None);
        }

        self.layers.push(Some(Layer::new(range)));
    }

    pub fn parse_layer(&mut self, s: &str) {
        let mut iter = s.split_whitespace();

        let depth = iter.next().unwrap().trim_matches(':').parse().unwrap_or(0);
        let range = iter.next().unwrap().parse().unwrap_or(0);

        self.add_layer(depth, range);
    }

    pub fn trip_severity(&self) -> u32 {
        let steps = self.layers.len() as u32;

        let mut severity = 0;

        for t in 0..steps {
            if let Some(ref l) = self.layers[t as usize] {
                if l.scanner_hit(t) {
                    severity += t * l.range;
                }
            }
        }

        severity
    }

    pub fn safe_trip_delay(&self) -> u32 {
        let mut delay = 0;

        while !self.is_trip_safe(delay) {
            delay += 1;
        }

        delay
    }

    fn is_trip_safe(&self, delay: u32) -> bool {
        let steps = self.layers.len() as u32;

        for t in 0..steps {
            if let Some(ref l) = self.layers[t as usize] {
                if l.scanner_hit(delay + t) {
                    return false;
                }
            }
        }

        true
    }
}

struct Layer {
    range: u32,
}

impl Layer {
    fn new(range: u32) -> Self {
        Layer { range }
    }

    fn scanner_hit(&self, t: u32) -> bool {
        t % ((self.range - 1) * 2) == 0
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

    #[test]
    fn calculate_delay_for_safe_trip() {
        let mut firewall = Firewall::new();

        firewall.add_layer(0, 3);
        firewall.add_layer(1, 2);
        firewall.add_layer(4, 4);
        firewall.add_layer(6, 4);

        assert_eq!(10, firewall.safe_trip_delay());
    }
}
