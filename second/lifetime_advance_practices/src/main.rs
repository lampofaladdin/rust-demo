// #[derive(Debug)]
// struct Foo;

// impl Foo {
//     fn mutate_and_share(&mut self) -> &Self {
//         &*self
//     }
//     fn share(&self) {}
// }

// fn main() {
//     let mut foo = Foo;
//     let loan = foo.mutate_and_share();
//     foo.share();
//     println!("{:?}", loan);
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::HashMap;
//     use std::hash::Hash;
//     fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
//     where
//         K: Clone + Eq + Hash,
//         V: Default,
//     {
//         match map.get_mut(&key) {
//             Some(value) => value,
//             None => {
//                 map.insert(key.clone(), V::default());
//                 map.get_mut(&key).unwrap()
//             }
//         }
//     }
// }
// #![allow(unused)]
// fn main() {
//     fn fn_elision(x: &i32) -> &i32 {
//         x
//     }
//     let closure_slision = |x: &i32| -> &i32 { x };
// }

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
}
