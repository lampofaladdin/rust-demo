// use std::fmt::Result;
// use std::io::Result as IOResult;

// fn main() {}

// 使用两种方式填空
// 不要添加新的代码行
// use std::collections::*;
// use std::collections::{BTreeMap, HashMap, HashSet};

// fn main() {
//     let _c1: HashMap<&str, i32> = HashMap::new();
//     let mut c2 = BTreeMap::new();
//     c2.insert(1, "a");
//     let _c3: HashSet<i32> = HashSet::new();
// }

// pub mod a {
//     pub const I: i32 = 3;

//     fn semisecret(x: i32) -> i32 {
//         use self::b::c::J;
//         x + J
//     }

//     pub fn bar(z: i32) -> i32 {
//         semisecret(I) * z
//     }
//     pub fn foo(y: i32) -> i32 {
//         semisecret(I) + y
//     }

//     mod b {
//         pub(in crate::a) mod c {
//             pub(in crate::a) const J: i32 = 4;
//         }
//     }
// }

// fn main() {}

// use hello_package;

// fn main() {
//     assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
//     assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
// }
