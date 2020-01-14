extern crate rand;

pub mod markov_text_generator;
mod weighted_char;
mod weighted_char_set;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
