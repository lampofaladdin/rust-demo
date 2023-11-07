// // 填空
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // 实现下面的代码
//         return;
//     }
//     panic!("wrong words");

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink("lemonad1e");

//     println!("Exercise Failed if printing out this line!");
// }

// 修复所有的 panic，让代码工作
// fn main() {
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);

//     let v = vec![1, 2, 3];
//     let ele = v[2];
//     let ele = v.get(2).unwrap();

//     // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
//     let v = production_rate_per_hour(2);

//     divide(15, 1);

//     println!("Success!")
// }

// fn divide(x: u8, y: u8) {
//     println!("{}", x / y)
// }

// fn production_rate_per_hour(speed: u32) -> f64 {
//     let cph: u32 = 221;
//     match speed {
//         1..=4 => (speed * cph) as f64,
//         5..=8 => (speed * cph) as f64 * 0.9,
//         9..=10 => (speed * cph) as f64 * 0.77,
//         _ => 0 as f64,
//     }
// }

// pub fn working_items_per_minute(speed: u32) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }

// use std::{
//     fs::File,
//     io::{Error, ErrorKind, Read},
// };

// fn main() {
//     //     let f = File::open("hello.txt");
//     //     let mut f = match f {
//     //         Ok(file) => file,
//     //         Err(e) => match e.kind() {
//     //             ErrorKind::NotFound => match File::create("hello.txt") {
//     //                 Ok(fc) => fc,
//     //                 Err(e) => {
//     //                     panic!("{}", e);
//     //                 }
//     //             },
//     //             ohter => {
//     //                 panic!("{}", ohter);
//     //             }
//     //         },
//     //     };
//     //     let mut file_text = String::new();
//     //     f.read_to_string(&mut file_text);
//     //     println!("{:?}", file_text);
//     // let f = File::open("hello.txt").expect("file to open hello.txt");
//     {
//         return;
//     }
//     println!("hello");
// }

use std::{
    error::Error,
    fs::File,
    io::{Error, Read},
};

// fn read_user_name() -> Result<String, Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut name = String::new();
//     match f.read_to_string(&mut name) {
//         Ok(_) => {
//             if name.trim().is_empty() {
//                 Ok("aladdin".to_string())
//             } else {
//                 Ok(name)
//             }
//         }
//         Err(e) => Err(e),
//     }
// }

fn read_user_name() -> Result<String, Box<dyn Error>> {
    let mut name = String::new();
    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)
}
fn main() {
    match read_user_name() {
        Ok(s) => {
            println!("{}", s);
        }
        Err(e) => {
            println!("{:?}", e.try_into());
        }
    }
}
