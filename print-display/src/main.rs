use std::fmt::{self, write};

#[derive(Debug)]
struct MinMax(i64, i64);

 

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} ,y : {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Compare Struct");
    println!("Dsiplay: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "the big range is {big},the small range is {small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points");
    println!("Dsiplay: {}", point);
    println!("Debug: {:?}", point);
}
