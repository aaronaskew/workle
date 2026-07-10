const GAMES: &str = include_str!("../data/games.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let  guesser = workle::algorithms::Naive::new();
        workle::play(answer, guesser);
    }

    println!("Hello, world!");
}
