use std::{env, error::Error, fs};

const CONFIG_FLAG_CASE_INSENSITIVE: u32 = 0x01;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub flag: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let mut flag = 0;

        let case_insensitive = env::var("CASE_INSENSITIVE")
            .unwrap_or("0".to_string())
            .parse::<u32>()
            .expect("environment variable parse error");

        if case_insensitive != 0 {
            flag |= CONFIG_FLAG_CASE_INSENSITIVE;
        }

        Ok(Config {
            query,
            filename,
            flag,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let case_insensitive = config.flag & CONFIG_FLAG_CASE_INSENSITIVE != 0;

    let results = if case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents.lines().filter(|l| l.contains(query)).collect();
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
