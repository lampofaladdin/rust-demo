use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("搜索到的内容：{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // 原始代码
    // let mut result_vec = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         result_vec.push(line);
    //     }
    // }
    // result_vec
    // 变更后的代码
    content.lines().filter(|c| c.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // 原始代码
    // let mut result_vec = Vec::new();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         result_vec.push(line);
    //     }
    // }
    // result_vec

    // 迭代器以后的代码
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub struct MyErr {
    pub msg: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, MyErr> {
        if args.len() < 3 {
            return Err(MyErr {
                msg: String::from("参数不够"),
            });
        }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(MyErr {
                    msg: String::from("没有获取到query参数"),
                })
            }
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(MyErr {
                    msg: String::from("没有获取到filename参数"),
                })
            }
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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
