use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread, time,
};

use sigle_web_pracitices::ThreadPool;

fn main() {
    println!("123");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handler_connection(stream);
        });
    }
}

fn handler_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    let first_line = buffer_reader.lines().next().unwrap().unwrap();
    if first_line == "GET / HTTP/1.1" {
        thread::sleep(time::Duration::from_secs(5));
        let str = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK \r\n\r\n {str}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let str = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND \r\n\r\n {str}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
