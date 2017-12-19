pub struct Spinlock {
    steps: usize,
    buffer: Vec<u32>,
    position: usize,
}

impl Spinlock {
    pub fn new(steps: usize) -> Self {
        Spinlock {
            steps,
            buffer: vec![0],
            position: 0,
        }
    }

    pub fn spin(&mut self, times: u32) -> Vec<u32> {
        for i in 1..(times + 1) {
            self.position = (self.position + self.steps) % self.buffer.len() + 1;

            if self.position >= self.buffer.len() {
                self.buffer.push(i);
            } else {
                self.buffer.insert(self.position, i);
            }
        }

        self.buffer.clone()
    }

    pub fn value_after_latest(&mut self, spins: u32) -> u32 {
        self.spin(spins);

        let latest = self.buffer.iter().position(|&i| i == spins).unwrap_or(0);

        self.buffer
            .get((latest + 1) % self.buffer.len())
            .cloned()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        assert_eq!(vec![0], Spinlock::new(3).spin(0));
    }

    #[test]
    fn spin_once() {
        assert_eq!(vec![0, 1], Spinlock::new(3).spin(1));
    }

    #[test]
    fn spin_twice() {
        assert_eq!(vec![0, 2, 1], Spinlock::new(3).spin(2));
    }

    #[test]
    fn spin_three_times() {
        assert_eq!(vec![0, 2, 3, 1], Spinlock::new(3).spin(3));
    }

    #[test]
    fn spin_nine_times() {
        assert_eq!(vec![0, 9, 5, 7, 2, 4, 3, 8, 6, 1], Spinlock::new(3).spin(9));
    }

    #[test]
    fn get_value_after_latest() {
        assert_eq!(638, Spinlock::new(3).value_after_latest(2017));
    }
}
