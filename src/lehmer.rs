#[derive(Clone, Debug)]
pub struct Lehmer {
    state: u128
}

impl Lehmer {
    pub fn new() -> Self {
        Self {
            state: 128384995,
        }
    }

    pub fn random_float(&mut self, min: f64, max: f64) -> f64 {
        self.state *= 0xda942042e4dd58b5;

        return ((self.state >> 64) as u64) as f64 / std::u64::MAX as f64;
    }

    pub fn gen_range(&mut self, min: f64, max: f64) -> f64 {
        self.state *= 0xda942042e4dd58b5;

        return ((self.state >> 64) as u64) as f64 / std::u64::MAX as f64;
    }
}