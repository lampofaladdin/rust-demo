// fn main() {
//     let mut s = String::new();

//     let mut update_string = |str| s.push_str(str);
//     update_string("hello");

//     println!("{:?}", s);
// }

// fn main() {
//     let mut s = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}", s);
// }

// fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
//     f("hello")
// }
fn main() {
    let s = String::new();

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}
