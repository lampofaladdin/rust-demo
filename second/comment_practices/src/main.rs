use comment_practices::kinds::PrimaryColor;
use comment_practices::utils::mix;

fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}", mix(blue, yellow));
}
