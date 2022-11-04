use std::fmt::{self};

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} fine yingyingying ", self.0)
    }
}

fn main() {
    let d = Structure(2);
    println!("{:?}", d);
}
