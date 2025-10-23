use std::cmp::{max, min};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinMaxTs {
    pub min: u64,
    pub max: u64,
}

impl MinMaxTs {
    pub fn new() -> MinMaxTs {
        // MAX MIN is used to convenience of min max functions usage
        MinMaxTs {
            min: u64::MAX,
            max: u64::MIN,
        }
    }

    pub fn set(&mut self, ts: u64) {
        self.min = min(self.min, ts);
        self.max = max(self.max, ts);
    }

    pub fn merge(&mut self, other: &MinMaxTs) {
        self.min = min(other.min, self.min);
        self.max = max(other.max, self.max);
    }
}

impl Default for MinMaxTs {
    fn default() -> Self {
        Self::new()
    }
}
