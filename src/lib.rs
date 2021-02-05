use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process,
};

pub struct Config {
    pub words_file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let words_file = args[1].clone();

        Ok(Config { words_file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dictionary = lines_from_file(config.words_file).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let words_to_test = vec!["GYM", "FISH", "ELIDE", "GLIDE", "DELIRIUM"];
    for word in actual_words(dictionary, words_to_test) {
        println!("Dictionary has: {}!", word);
    }

    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn actual_words(dictionary: Vec<String>, words_to_test: Vec<&str>) -> Vec<String> {
    let mut actual_words = Vec::new();

    for word in words_to_test {
        let word_to_search = String::from(word);
        match dictionary.binary_search(&word_to_search) {
            Ok(_) => actual_words.push(word_to_search),
            Err(_) => (),
        }
    }
    actual_words
}
