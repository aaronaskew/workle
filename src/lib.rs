use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../data/wordle-weighted.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.split_whitespace().step_by(2)),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        // play six rounds where it invokes guesser each round
        let mut history = Vec::new();

        // Wordle allows 6, we allow more for algorithm analysis
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            assert!(self.dictionary.contains(guess.as_str()));
            if guess == answer {
                return Some(i);
            }

            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }

        None
    }
}

impl Default for Wordle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert!(answer.len() == 5);
        assert!(guess.len() == 5);

        let mut correctness = [Correctness::Wrong; 5];

        // Check for GREEN characters (character + location)
        for (i, (char_guess, char_answer)) in guess.chars().zip(answer.chars()).enumerate() {
            if char_guess == char_answer {
                correctness[i] = Correctness::Correct;
            }
        }

        // Check for YELLOW characters

        // which answer characters are already used to mark the guess?
        let mut marked_answer_chars = [false; 5];
        // set the green chars as marked
        for (i, &c) in correctness.iter().enumerate() {
            if c == Correctness::Correct {
                marked_answer_chars[i] = true;
            }
        }

        for (i, char_guess) in guess.char_indices() {
            if correctness[i] == Correctness::Correct {
                continue;
            }

            if answer.chars().enumerate().any(|(j, char_answer)| {
                if char_guess == char_answer && !marked_answer_chars[j] {
                    marked_answer_chars[j] = true;
                    return true;
                }

                false
            }) {
                correctness[i] = Correctness::Misplaced;
            }
        }

        correctness
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "right".to_string() });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|history| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {
                Correctness::Correct
            };
            (M) => {
                Correctness::Misplaced
            };
            (W) => {
                Correctness::Wrong
            };

            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "zzzzz"), mask![W W W W W]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "bcdea"), mask![M M M M M]);
        }

        #[test]
        fn one_green() {
            assert_eq!(Correctness::compute("abcde", "azzzz"), mask![C W W W W]);
        }

        #[test]
        fn green_priority_over_yellow() {
            assert_eq!(Correctness::compute("aabbb", "bbbbb"), mask![W W C C C])
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aaabb", "aaaaa"), mask![C C C W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aaabb", "cccaa"), mask![W W W M M]);
        }
    }
}
