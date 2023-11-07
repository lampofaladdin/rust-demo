// /* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */
// // `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// // 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
// fn main() {
//     let i = 3;
//     {
//         let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
//                           //                                                │
//         println!("borrow1: {}", borrow1); //              │
//     } // `borrow1` 生命周期结束. ──────────────────────────────────┘
//     {
//         let borrow2 = &i;

//         println!("borrow2: {}", borrow2);
//     }
// }

// // 引用参数中的生命周期 'a 至少要跟函数活得一样久
// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// // 可变引用依然需要标准生命周期
// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// // 下面代码中，每个参数都拥有自己独立的生命周期，事实上，这个例子足够简单，因此它们应该被标记上相同的生命周期 `'a`，但是对于复杂的例子而言，独立的生命周期标注是可能存在的
// fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("`print_multi`: x is {}, y is {}", x, y);
// }

// // 返回一个通过参数传入的引用是很常见的，但是这种情况下需要标注上正确的生命周期
// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn main() {
//     let x = 7;
//     let y = 9;

//     print_one(&x);
//     print_multi(&x, &y);

//     let z = pass_x(&x, &y);
//     print_one(z);

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }

// /* 添加合适的生命周期标注，让下面的代码工作 */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

// /* 使用三种方法修复下面的错误  */
// fn invalid_output() -> String {
//     String::from("foo")
// }
/* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>(s: &'a str) -> &'a str {
//     s
// }
/* 使用三种方法修复下面的错误  */
// fn invalid_output() ->&'static str {
//     "foo"
// }

// fn main() {}

// // `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// /* 让下面的代码工作 */
// fn failed_borrow() {
//     let _x = 12;

//     // ERROR: `_x` 活得不够久does not live long enough
//     let y = &_x;

//     // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
//     // 你不能将一个小的生命周期强转成大的
// }

// fn main() {
//     let (four, nine) = (4, 9);

//     print_refs(&four, &nine);
//     // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

//     failed_borrow();
//     // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
// }

/* 增加合适的生命周期标准，让代码工作 */

// // `i32` 的引用必须比 `Borrowed` 活得更久
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     {
//         let y = 15;
//         {
//             let single = Borrowed(&x);
//             let double = NamedBorrowed { x: &x, y: &y };
//             let reference = Either::Ref(&x);
//             let number = Either::Num(y);
//             println!("x is borrowed in {:?}", single);
//             println!("x and y are borrowed in {:?}", double);
//             println!("x is borrowed in {:?}", reference);
//             println!("y is *not* borrowed in {:?}", number);
//         }
//     }
// }

// /* 让代码工作 */
// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn main() {
//     let var_a = 35;
//     let example: Example;
//     let var_b = NoCopyType {};
//     {
//         /* 修复错误 */
//         example = Example {
//             a: &var_a,
//             b: &var_b,
//         };
//     }
//     println!("(Success!) {:?}", example);
// }

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// // /* 修复函数的签名 */
// // fn fix_me<'a: 'b, 'b>(foo: &'a Example) -> &'b NoCopyType {
// //     foo.b
// // }

// /* 修复函数的签名 */
// fn fix_me<'a, 'b>(foo: &'a Example) -> &'b NoCopyType
// where
//     'a: 'b,
// {
//     foo.b
// }

// fn main() {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }

// struct Owner(i32);

// impl Owner {
//     fn add_one<'a>(&'a mut self) {
//         self.0 += 1;
//     }
//     fn print<'a>(&'a self) {
//         println!("`print`: {}", self.0);
//     }
// }

// fn main() {
//     let mut owner = Owner(18);

//     owner.add_one();
//     owner.print();
// }

/* 添加合适的生命周期让下面代码工作 */
// struct ImportantExcerpt {
//     part: str,
// }

// impl ImportantExcerpt {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// fn main() {}

// /* 添加合适的生命周期让下面代码工作 */
// // 尼玛耶,这语法真的是越来越日怪了.
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&'a self) -> i32 {
//         3
//     }
// }

// fn main() {}

/* 移除所有可以消除的生命周期标注 */

// fn nput(x: &i32) {
//     println!("`annotated_input`: {}", x);
// }

// fn pass(x: &i32) -> &i32 {
//     x
// }

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// struct Owner(i32);

// impl Owner {
//     fn add_one(&mut self) {
//         self.0 += 1;
//     }
//     fn print(&self) {
//         println!("`print`: {}", self.0);
//     }
// }

// struct Person<'a> {
//     age: u8,
//     name: &'a str,
// }

// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {}

// 总结
// 生命周期的作用是期望在编译阶段就能解决悬垂引用的问题
// 标记生命周期不会改变代码本身的作用域,只是告诉编译器,应该怎么理解这段代码,让编译器不为难我们,鲁迅说的,手动狗头
// 生命周期消除的规则
// 每一个引用入参都有对应的参数生命周期,如果没有引用出参,不需要标记手动生命周期
// 如果只有一个引用入参,一个引用出参 不需要标注
// 如果方法入参有Self类型的,那么认为他的默认生命周期是Self的生命周期,如果返回的结果不是Self的生命周期,需要手动标注
// 方法中的生命周期标注好日怪,emm,好像泛型也是这么标注的
// impl<'a> Fine<'a> {
//  fn demo(&'a slef)-> i32{
//    123
//  }
// }
// 约束生命周期的表示 'a:'b,表示 'a 的生命周期 大于 'b 的生命周期
// 语法越来越日怪了
// 静态生命周期 'static
// 表示程序全局生命周期,尽可能不用,除非是实在无法写出该代码的生命周期

fn main() {}
use std::{fmt::Display, path::Display};

/**
 * 这么日怪的语法?????,操了个DJ
 * 生命周期的标注,类似于泛型的标注...
 * 特么的,泛型的生命周期 应该怎么写???
 */
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/**
 * 泛型生命周期的语法,行吧.
 * 是真特么6
 */
fn demo<'a, T: std::cmp::Ord>(x: &'a T, y: &'a T) -> &'a T {
    if x > y {
        return x;
    }
    y
}
