use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("搜索到的内容：{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result_vec = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result_vec.push(line);
        }
    }
    result_vec
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub struct MyErr {
    pub msg: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, MyErr> {
        if args.len() < 3 {
            return Err(MyErr {
                msg: String::from("参数不够"),
            });
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
