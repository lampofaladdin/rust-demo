// /// `add_one` 将指定值加1
// ///
// /// # Examples11
// ///
// /// ```
// /// let arg = 5;
// /// let answer = comment_practices::add_two(arg);
// ///
// /// assert_eq!(7, answer);
// /// ```
// pub fn add_two(x: i32) -> i32 {
//     x + 2
// }

// /// # Panics
// ///
// /// The function panics if the second argument is zero.
// ///
// /// ```rust,should_panic
// /// // panics on division by zero
// /// comment_practices::div(10, 0);
// /// ```
// pub fn div(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         panic!("Divide-by-zero error");
//     }

//     a / b
// }

// pub mod a {
//     /// `add_one` 返回一个[`Option`]类型
//     /// 跳转到[`crate::MySpecialFormatter`]
//     pub fn add_one(x: i32) -> Option<i32> {
//         Some(x + 1)
//     }
// }

// pub struct MySpecialFormatter;

// /// `add_one` 返回一个[`Option`]类型
// pub fn add_one(x: i32) -> Option<i32> {
//     Some(x + 1)
// }

// use std::sync::mpsc::Receiver;

// /// [`Receiver<T>`]   [`std::future`].
// ///
// ///  [`std::future::Future`] [`Self::recv()`].
// pub struct AsyncReceiver<T> {
//     sender: Receiver<T>,
// }

// impl<T> AsyncReceiver<T> {
//     pub async fn recv() -> T {
//         unimplemented!()
//     }
// }

// /// 跳转到结构体  [`Foo`](struct@Foo)
// pub struct Bar;

// /// 跳转到同名函数 [`Foo`](fn@Foo)
// pub struct Foo {}

// /// 跳转到同名宏 [`foo!`]
// pub fn Foo() {}

// #[macro_export]
// macro_rules! foo {
//     () => {};
// }

// #[doc(alias = "x")]
// #[doc(alias = "big")]
// pub struct BigX;

// #[doc(alias("y", "big"))]
// pub struct BigY;

//! # Art
//!
//!  未来的艺术建模库，现在的调色库

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //! 定义颜色的类型

    /// 主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 副色
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! 实用工具，目前只实现了调色板
    use crate::kinds::*;

    /// 将两种主色调成副色
    /// ```rust
    /// use comment_practices::utils::mix;
    /// use comment_practices::kinds::{PrimaryColor,SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
