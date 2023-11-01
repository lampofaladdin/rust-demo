// struct Container(i32, i32);

// // 使用关联类型实现重新实现以下特征
// // trait Contains {
// //    type A;
// //    type B;

// trait Contains {
//     type A;
//     type B;
//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains for Container {
//     type A = i32;
//     type B = i32;
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }
//     // Grab the first number.
//     fn first(&self) -> i32 {
//         self.0
//     }

//     // Grab the last number.
//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// fn difference<C: Contains>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!(
//         "Does container contain {} and {}: {}",
//         &number_1,
//         &number_2,
//         container.contains(&number_1, &number_2)
//     );
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// use std::ops::Sub;

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T> Sub for Point<T>
// where
//     T: Sub<Output = T>,
// {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!")
// }

// use std::{ops::Sub, process::Output};

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!")
// }

// use std::{ops::Sub, process::Output};

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T: Sub<Output = T>> Sub<Self> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!")
// }

// trait UsernameWidget {
//     fn get(&self) -> String;
// }

// trait AgeWidget {
//     fn get(&self) -> u8;
// }

// struct Form {
//     username: String,
//     age: u8,
// }

// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }

// impl AgeWidget for Form {
//     fn get(&self) -> u8 {
//         self.age
//     }
// }

// fn main() {
//     let form = Form {
//         username: "rustacean".to_owned(),
//         age: 28,
//     };

//     // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
//     // 毕竟，这里有好几个同名的 `get` 方法
//     //
//     // println!("{}", form.get());

//     let username = UsernameWidget::get(&form);
//     assert_eq!("rustacean".to_owned(), username);
//     let age = <Form as AgeWidget>::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
//     assert_eq!(28, age);

//     println!("Success!")
// }

// trait Pilot {
//     fn fly(&self) -> String;
// }

// trait Wizard {
//     fn fly(&self) -> String;
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) -> String {
//         String::from("This is your captain speaking.")
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) -> String {
//         String::from("Up!")
//     }
// }

// impl Human {
//     fn fly(&self) -> String {
//         String::from("*waving arms furiously*")
//     }
// }

// fn main() {
//     let person = Human;

//     assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
//     assert_eq!(<Human as Wizard>::fly(&person), "Up!");

//     assert_eq!(person.fly(), "*waving arms furiously*");

//     println!("Success!")
// }

// trait Person {
//     fn name(&self) -> String;
// }

// // Person 是 Student 的 supertrait .
// // 实现 Student 需要同时实现 Person.
// trait Student: Person {
//     fn university(&self) -> String;
// }

// trait Programmer {
//     fn fav_language(&self) -> String;
// }

// // CompSciStudent (computer science student) 是 Programmer
// // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }

// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }

// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String,
// }

// // 为 CSStudent 实现所需的特征
// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         self.git_username.to_string()
//     }
// }

// impl Person for CSStudent {
//     fn name(&self) -> String {
//         self.name.to_string()
//     }
// }

// impl Student for CSStudent {
//     fn university(&self) -> String {
//         self.university.to_string()
//     }
// }

// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         self.fav_language.to_string()
//     }
// }

// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string(),
//     };

//     // 填空
//     println!("{}", comp_sci_student_greeting(&student));
// }

use std::fmt;

// 定义一个 newtype `Pretty`
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
