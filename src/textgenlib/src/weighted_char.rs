use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

pub struct WeightedChar {
    data: char,
    weight: u32,
}

impl WeightedChar {
    pub fn new(data: char) -> Self {
        WeightedChar {
            data: data,
            weight: 0,
        }
    }

    pub fn new_with_weight(data: char, weight: u32) -> Self {
        WeightedChar {
            data: data,
            weight: weight,
        }
    }

    pub fn get_char(&self) -> char {
        self.data
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }
}

impl PartialEq for WeightedChar {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
}

impl Eq for WeightedChar {}

impl Hash for WeightedChar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}
