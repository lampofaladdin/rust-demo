use std::mem;

fn analyeze_slice(slice: &[i32]) {
    println!("first element of slice: {}", slice[0]);
    println!("the slice has {} members", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500];

    println!("the first element of array: {}", xs[0]);
    println!("the second element of array: {}", xs[1]);
    println!("the lenth of array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow whole array as slice ");
    analyeze_slice(&xs);

    println!("borrow a section of array as slice");
    analyeze_slice(&ys[1..4]);

    println!("{}", xs[5]);
}
