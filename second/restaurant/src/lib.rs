// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         pub fn seat_at_table() {}
//     }
//     pub mod serving {
//         pub fn take_order() {}
//         pub fn server_order() {}
//         pub fn take_payment() {}
//         pub fn complain() {}
//     }
// }

// // in lib.rs

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // 使用三种方式填空
//         //1. 使用关键字 `super`
//         //2. 使用绝对路径
//         super::front_of_house::serving::server_order();
//         crate::front_of_house::serving::server_order();
//     }

//     fn cook_order() {}
// }

// pub fn eat_at_restaurant() {
//     // 使用绝对路径调用
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 使用相对路径调用
//     self::front_of_house::hosting::seat_at_table();
// }

mod back_of_hose;
mod front_of_house;
pub use crate::back_of_hose::back_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    crate::front_of_house::hosting::add_to_waitlist();

    crate::back_of_house::cook_order();

    String::from("yummy yummy!")
}
