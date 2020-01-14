use std::collections::HashMap;
use std::str::Chars;

use crate::weighted_char::WeightedChar;
use crate::weighted_char_set::WeightedCharSet;

pub struct MarkovTextGenerator {
    chain: HashMap<char, WeightedCharSet>,
}

impl MarkovTextGenerator {
    pub fn new() -> Self {
        Self {
            chain: HashMap::new(),
        }
    }

    pub fn load(&mut self, text: &str) {
        Self::load_text(self, &mut text.chars());
    }

    fn load_text(&mut self, chars_itr: &mut Chars) {
        let first = chars_itr.next();
        match first {
            None => return,
            Some(value) => {
                let mut prev = value;
                for c in chars_itr {
                    if !self.chain.contains_key(&prev) {
                        self.chain.insert(prev, WeightedCharSet::new());
                    }
                    let mut chain = self.chain.remove(&prev).unwrap();
                    Self::update(&mut chain, WeightedChar::new_with_weight(c, 1));
                    self.chain.insert(prev, chain);
                    prev = c;
                }
            }
        }
    }

    fn update(chain: &mut WeightedCharSet, weighted_char: WeightedChar) {
        let mut weight = 0;
        let existing = chain.get(&weighted_char);
        match existing {
            None => {
                chain.insert(weighted_char);
                return;
            }
            Some(old) => {
                weight += old.get_weight() + 1;
            }
        }

        chain.remove(&weighted_char);
        chain.insert(WeightedChar::new_with_weight(
            weighted_char.get_char(),
            weight,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        let mut markov = MarkovTextGenerator::new();
        markov.load("abc");
        assert_eq!(markov.chain.contains_key(&'a'), true);
        assert_eq!(markov.chain.contains_key(&'b'), true);
        assert_eq!(markov.chain.contains_key(&'c'), false);
    }

    #[test]
    fn validate_simple_chain() {
        let mut markov = MarkovTextGenerator::new();
        markov.load("abc");

        match markov.chain.remove(&'a') {
            Some(next) => {
                assert_eq!(next.contains(&WeightedChar::new('b')), true);
            }
            None => panic!(format!("Expected to find 'b' in the next chain.")),
        }
    }

    #[test]
    fn weight_increases() {
        let mut markov = MarkovTextGenerator::new();
        markov.load("ababab");

        match markov.chain.remove(&'a') {
            Some(next) => match next.get(&WeightedChar::new('b')) {
                Some(value) => assert_eq!(value.get_weight(), 3),
                None => panic!(format!("Expected to find a WeightedChar for 'b'")),
            },
            None => panic!(format!("Expected to find 'b' in the next chain.")),
        }
    }
}
