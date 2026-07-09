use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    n_grams_path: PathBuf,
    #[arg(short)]
    wordle_words_path: PathBuf,
    #[arg(short)]
    filtered_n_grams_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let n_grams_path = if args.n_grams_path.is_relative() {
        std::env::current_dir().unwrap().join(args.n_grams_path)
    } else {
        args.n_grams_path
    };

    let wordle_words_path = if args.wordle_words_path.is_relative() {
        std::env::current_dir()
            .unwrap()
            .join(args.wordle_words_path)
    } else {
        args.wordle_words_path
    };

    let filtered_n_grams_path = if args.filtered_n_grams_path.is_relative() {
        std::env::current_dir()
            .unwrap()
            .join(args.filtered_n_grams_path)
    } else {
        args.filtered_n_grams_path
    };

    let n_grams_reader = BufReader::new(File::open(n_grams_path)?);

    let wordle_words_reader = BufReader::new(File::open(wordle_words_path)?);

    let mut output_file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filtered_n_grams_path)?;

    let wordle_words = wordle_words_reader
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                line
            } else {
                panic!("can't read wordle words")
            }
        })
        .collect::<HashSet<_>>();

    for line in n_grams_reader.lines().map_while(Result::ok) {
        let word = line.split(' ').next().unwrap().to_string();

        if wordle_words.contains(&word) {
            writeln!(output_file, "{}", line)?;
        }
    }

    Ok(())
}
