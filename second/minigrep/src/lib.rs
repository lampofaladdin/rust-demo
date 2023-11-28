use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(i) => i,
            None => return Err("not found query"),
        };

        let file_path = match args.next() {
            Some(i) => i,
            None => return Err("not found file path"),
        };

        let ignore_case = if env::var("IGNORE_CASE").is_ok() {
            env::var("IGNORE_CASE").is_ok_and(|x| x == "1")
        } else {
            args.next().unwrap_or("".to_string()).eq("IGNORE_CASE=1")
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in &result {
        println!("{line}");
    }
    let result = result.into_iter().map(|x| x.to_string()).collect();
    Ok(result)
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
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
