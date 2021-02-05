use itertools::Itertools;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process,
};

pub struct Config {
    pub words_file: String,
    pub min_size: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: cargo run <words_filename> <min_size_of_words>.");
        }

        let words_file = args[1].clone();
        let min_size: usize = args[2].parse().unwrap();

        Ok(Config {
            words_file,
            min_size,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dictionary = lines_from_file(&config.words_file).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let mut _input = String::new();
    // println!("Please enter some letters: ");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line.");

    let letters = vec!['A', 'C', 'R', 'G', 'D', 'E'];
    let words_to_test = generate_permutations(&letters, &config.min_size);
    for word in actual_words(dictionary, words_to_test) {
        println!("Dictionary has: {}!", word);
    }

    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn actual_words(dictionary: Vec<String>, words_to_test: Vec<String>) -> Vec<String> {
    let mut actual_words = Vec::new();

    for word in words_to_test {
        match dictionary.binary_search(&word) {
            Ok(_) => actual_words.push(word),
            Err(_) => (),
        }
    }
    actual_words
}

fn generate_permutations(letters: &Vec<char>, min_size: &usize) -> Vec<String> {
    let mut permutations: Vec<String> = Vec::new();
    let mut vec_sizes: Vec<usize> = Vec::new();

    let mut cur_size = *min_size;
    while cur_size <= letters.len() {
        vec_sizes.push(cur_size);
        cur_size = cur_size + 1;
    }

    for num_letters in vec_sizes {
        for permutation in letters.iter().permutations(num_letters).unique() {
            let word: String = permutation.into_iter().collect();
            permutations.push(word);
        }
    }
    permutations
}
