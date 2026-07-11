use workle::Wordle;

const GAMES: &str = include_str!("../data/games.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = workle::algorithms::Naive::new();
        let wordle = Wordle::new();
        wordle.play(answer, guesser);
    }

    println!("Hello, world!");
}
