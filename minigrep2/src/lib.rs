extern crate getopts;
use getopts::Options;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let context = "\
test
Test
TEST
tesT";

        let query = "test";
        assert_eq!(search(query, context), vec!["test"])
    }

    #[test]
    fn test_sensitive_search() {
        let context = "\
test
Test
TEST
tesT";

        let query = "test";
        assert_eq!(
            sensitive_search(query, context),
            vec!["test", "Test", "TEST", "tesT"]
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let context = fs::read_to_string(config.file_name)?;

    let results = if config.sensitive {
        sensitive_search(&config.query, &context)
    } else {
        search(&config.query, &context)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in context.lines() {
        if line.contains(&query) {
            results.push(line)
        }
    }
    results
}

pub fn sensitive_search<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in context.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line)
        }
    }
    results
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args!!");
        }

        let mut opts = Options::new();
        opts.optopt("s", "SENSITIVE", "do sensitive search", "sensitive_search ");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        let query = args[1].clone();
        let file_name = args[2].clone();
        let sensitive = matches.opt_present("s");
        Ok(Config {
            query,
            file_name,
            sensitive,
        })
    }
}
