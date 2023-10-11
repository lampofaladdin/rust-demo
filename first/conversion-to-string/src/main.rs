use std::fmt;

struct Circle {
    raduis: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of raduis {}", self.raduis)
    }
}

fn main() {
    let circle = Circle { raduis: 8 };
    println!("{}", circle);

    let parsed: i32 = "5".parse().unwrap();

    let turbo_parsed = "10".parse::<i32>().unwrap();
}
