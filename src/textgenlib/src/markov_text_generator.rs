use std::collections::{HashMap, HashSet};
use std::str::Chars;

use crate::weighted_char::WeightedChar;

pub struct MarkovTextGenerator {
    chain: HashMap<char, HashSet<WeightedChar>>,
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
                        self.chain.insert(prev, HashSet::new());
                    }
                    let mut chain = self.chain.remove(&prev).unwrap();
                    Self::update(&mut chain, WeightedChar::new_with_weight(c, 1));
                    self.chain.insert(prev, chain);
                    prev = c;
                }
            }
        }
    }

    fn update(chain: &mut HashSet<WeightedChar>, weighted_char: WeightedChar) {
        let existing = chain.get(&weighted_char);
        match existing {
            None => {
                chain.insert(weighted_char);
            }
            Some(old) => {
                let old_char = old.get_char();
                let old_weight = old.get_weight();
                chain.insert(WeightedChar::new_with_weight(old_char, old_weight + 1));
            }
        }
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
            _ => panic!(format!("Expected to find 'b' in the next chain.")),
        }
    }
}

/*


    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }


*/
