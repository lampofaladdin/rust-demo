fn main() {
    // let number = 3;
    // if number < 3 {
    //     println!("The value is less than 3")
    // } else {
    //     println!("The value is more than or equal 3");
    // }

    // let condition = true;
    // let number = if condition {
    //     let x = 1;
    //     x
    // } else {
    //     let y = 2;
    //     y
    // };
    // println!("The value of number {number}");

    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter > 100 {
            break counter;
        }
    };
    println!("The value of result {result}");
}
