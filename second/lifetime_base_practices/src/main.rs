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
