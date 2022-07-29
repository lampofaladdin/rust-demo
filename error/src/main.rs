// 越界访问会引起安全问题
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error),
//         },
//     };
// }

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     let f = read_username_from_file();
//     match f {
//         Ok(s) => println!("{}", s),
//         Err(e) => println!("{:?}", e),
//     }
// }
