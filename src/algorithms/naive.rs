use crate::{Guess, Guesser};

#[derive(Debug, Default)]
pub struct Naive;

impl Guesser for Naive {
    fn guess(&mut self, _history: &[Guess]) -> String {
        todo!()
    }
}

impl Naive {
    pub fn new() -> Self {
        Self
    }
}
