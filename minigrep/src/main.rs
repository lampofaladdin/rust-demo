use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("出现参数问题: {}", err.msg);
        process::exit(1)
    });

    println!("搜索内容：{}", config.query);
    println!("搜索文件:{} \n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("程序错误：{}", e);
        process::exit(1);
    }
}
