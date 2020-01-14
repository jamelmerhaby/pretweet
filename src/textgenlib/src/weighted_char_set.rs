use std::collections::HashSet;

use rand::Rng;

use crate::weighted_char::WeightedChar;

pub struct WeightedCharSet {
    values: HashSet<WeightedChar>,
    total_weight: u32,
}

impl WeightedCharSet {
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
            total_weight: 0,
        }
    }
    pub fn get(&self, value: &WeightedChar) -> Option<&WeightedChar> {
        self.values.get(value)
    }

    pub fn insert(&mut self, value: WeightedChar) -> bool {
        let weight = value.get_weight();
        let success = self.values.insert(value);
        if success {
            self.total_weight += weight;
        }

        success
    }

    pub fn contains(&self, value: &WeightedChar) -> bool {
        self.values.contains(value)
    }

    pub fn remove(&mut self, value: &WeightedChar) -> bool {
        self.values.remove(value)
    }

    pub fn get_random(&self) -> Option<&WeightedChar> {
        let mut range_generator = rand::thread_rng();
        let mut random_value = range_generator.gen_range(0, self.total_weight) + 1;

        for val in &self.values {
            random_value -= val.get_weight();
            if random_value <= 0 {
                return Some(val);
            }
        }

        None
    }
}
