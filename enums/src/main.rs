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

enum IpAddr {
    V4(Ipv)
}
