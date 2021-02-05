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
    let words_list = lines_from_file(config.words_file).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    for word in words_list {
        println!("Words list has: {}!", word);
    }

    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
