use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number { value: v }
    }
}

fn main() {
    // let num = Number::from(1);
    // println!("{:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("{:?}", num);
}
