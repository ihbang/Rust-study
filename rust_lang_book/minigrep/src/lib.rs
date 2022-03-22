use std::{env, error::Error, fs};

const CONFIG_FLAG_CASE_INSENSITIVE: u32 = 0x01;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub flag: u32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
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

    let search_function = if case_insensitive {
        search_case_insensitive
    } else {
        search
    };

    let results = search_function(&config.query, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
