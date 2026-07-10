pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    // play six rounds where it invokes guesser each round
    let mut history = Vec::new();

    // Wordle allows 6, we allow more for algorithm analysis
    for i in 1..=32 {
        let guess = guesser.guess(&history);
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

#[cfg(test)]
mod tests {
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
