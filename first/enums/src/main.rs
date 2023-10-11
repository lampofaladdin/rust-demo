// #[derive(Debug)]
// enum IpAddKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddKind::V4,
//         address: String::from("192.168.31.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddKind::V6,
//         address: String::from("192.168.31.2"),
//     };

//     dbg!("{:?}", home);
//     dbg!("{:?}", loopback);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("192.168.31.1"));
//     let loophack = IpAddr::V6(String::from("192.168.31.1"));
//     dbg!("{:?}", home);
//     dbg!("{:?}", loophack);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(255, 168, 31, 1);
//     let loophack = IpAddr::V6(String::from("192.168.31.1"));
//     dbg!("{:?}", home);
//     dbg!("{:?}", loophack);
// }

// enum IpAddr {
//     V4(Ipv)
// }

// use std::fmt::Debug;

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         dbg!(&self);
//     }
// }

// fn main() {
//     let msg = Message::Write(String::from("hello"));
//     let q = Message::Quit;
//     let m = Message::Move { x: 1, y: 2 };
//     let c = Message::ChangeColor(1, 2, 3);
//     c.call()
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//     let sum = x + y;
// }

// match 部分

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_of_cents(coin: Coin) -> i32 {
//     match coin {
//         Coin::Penny => {
//             println!("luck penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let a = value_of_cents(Coin::Penny);
//     println!("{}", a)
// }

// #[derive(Debug)]
// enum ZhState {
//     Shanghai,
//     Jiangsu,
// }

// enum Coin {
//     Penny,
//     Quarter(ZhState),
// }

// fn value_of_coin(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Quarter(state) => {
//             println!("the state is {:?}", state);
//             match state {
//                 ZhState::Jiangsu => 3,
//                 ZhState::Shanghai => 5,
//             }
//         }
//     }
// }

// fn main() {
//     let a = value_of_coin(Coin::Quarter(ZhState::Shanghai));
//     println!("{}", a)
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => Some(0),
//     }
// }

// fn main() {
//     let a = plus_one(None);
//     let b = plus_one(a);
//     let c = plus_one(b);
//     let d = plus_one(c);
//     print!("{:?}", d)
// }
fn add_hat() -> () {
    println!("add hat");
}
fn remove_hat() -> () {
    println!("remove hat");
}
fn other_fn(other: i32) -> () {
    println!("the number is {}", other);
}

fn the_nine() -> () {
    println!("other")
}

fn main() {
    let roll = 9;
    match roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => other_fn(other),
        9 => the_nine(),
    }
}
