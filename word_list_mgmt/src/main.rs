use anyhow::Result;
use clap::Parser;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
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
    output_path: Option<PathBuf>,
}

fn absolute_path(p: PathBuf) -> PathBuf {
    if p.is_relative() {
        std::env::current_dir().unwrap().join(p)
    } else {
        p
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let n_grams_path = absolute_path(args.n_grams_path);
    let wordle_words_path = absolute_path(args.wordle_words_path);

    let n_grams_reader = BufReader::new(File::open(n_grams_path)?);
    let wordle_words_reader = BufReader::new(File::open(wordle_words_path)?);

    let wordle_set = wordle_words_reader
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                line
            } else {
                panic!("can't read wordle words")
            }
        })
        .collect::<BTreeSet<_>>();

    // dbg!(&wordle_set);

    let n_grams_map = n_grams_reader
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                let mut it = line.split(' ');
                (
                    it.next().unwrap().to_string(),
                    it.next().unwrap().parse::<usize>().unwrap(),
                )
            } else {
                panic!("can't read n_grams")
            }
        })
        .collect::<HashMap<String, usize>>();

    // println!("w-n\n=====");

    // for word in wordle_set.difference(&n_grams_set) {
    //     println!("{}", word);
    // }

    // println!("\nn-w\n=====");

    // for word in n_grams_set.difference(&wordle_set) {
    //     println!("{}", word);
    // }

    // for line in n_grams_reader.lines().map_while(Result::ok) {
    //     let word = line.split(' ').next().unwrap().to_string();

    //     if wordle_words.contains(&word) {
    //         writeln!(output_file, "{}", line)?;
    //     }
    // }

    let mut wordle_count_map = BTreeMap::new();

    for word in wordle_set {
        let count = if let Some(count) = n_grams_map.get(&word) {
            *count
        } else {
            1
        };

        wordle_count_map.insert(word, count);
    }

    let total_count: usize = wordle_count_map.values().cloned().sum();

    let mut wordle_percent_map = BTreeMap::new();

    for (word, count) in &wordle_count_map {
        let percent = *count as f32 / total_count as f32 * 100.0;

        wordle_percent_map.insert(word, percent);
    }

    // // output wordle list with hard counts
    // if let Some(output_path) = args.output_path {
    //     let output_path = absolute_path(output_path);

    //     let mut output_file = File::options()
    //         .create(true)
    //         .truncate(true)
    //         .write(true)
    //         .open(output_path)?;

    //     for (word, count) in &wordle_count_map {
    //         writeln!(output_file, "{} {}", word, count)?;
    //     }
    // }

    // output wordle words with percentages
    if let Some(output_path) = args.output_path {
        let output_path = absolute_path(output_path);

        let mut output_file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(output_path)?;

        for (word, percent) in wordle_percent_map {
            writeln!(output_file, "{} {:.9}", word, percent)?;
        }
    }
    Ok(())
}
