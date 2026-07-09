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

        for (i, cg) in guess.chars().enumerate() {
            if cg == answer.chars().nth(i).unwrap() {
                correctness[i] = Correctness::Correct;
            } else if answer.contains(cg) {
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
