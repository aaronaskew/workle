const GAMES: &str = include_str!("../data/games.txt");

fn main() {
    let mut guesser = workle::algorithms::Naive::new();
    for answer in GAMES.split_whitespace() {
        workle::play(answer, guesser);
    }

    println!("Hello, world!");
}
