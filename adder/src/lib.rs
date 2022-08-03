// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
//     #[test]
//     fn another() {
//         panic!("it is an panic");
//     }
// }

// fn main() {}
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height < other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn smallr_cannot_hold_larger() {
//         let largetr = Rectangle {
//             width: 10,
//             height: 10,
//         };
//         let small = Rectangle {
//             width: 9,
//             height: 9,
//         };
//         assert!(!small.can_hold(&small));
//     }

//     #[test]
//     fn larger_can_hold_smaller() {
//         let largetr = Rectangle {
//             width: 10,
//             height: 10,
//         };
//         let small = Rectangle {
//             width: 9,
//             height: 9,
//         };
//         assert!(largetr.can_hold(&small));
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(result.contains("Carol"), "内容不包含 Carol ,{}", result);
//     }
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("小于1")
//         } else if value > 100 {
//             panic!("大于10101")
//         }
//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "大于100")]
//     fn gretter_than_100() {
//         Guess::new(200);
//     }
// }

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn it_wors() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("file fuck you "))
//         }
//     }
// }


// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
} 