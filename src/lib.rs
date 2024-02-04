use std::{env, fs};
use std::error::Error;

use colored::{Color, Colorize};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub grep_color: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let grep_color = env::var("MINI_GREP_COLOR").unwrap_or_else(|_| String::from("White"));

        Ok(Config { query, file_path, ignore_case, grep_color })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line.color(Color::from(config.grep_color.to_string())));
    }

    Ok(())
}

// 大小写敏感
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()
}