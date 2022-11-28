#![allow(overflowing_literals)]
fn main() {
    let decimal = 64.123123;

    // let integer: u8 = decimal;

    let integer = decimal as u8;

    let character = integer as char;

    println!("Cast :{}=> {} => {}", decimal, integer, character);

    println!("1000 as a u8 is :{}", 1000 as u8);

    println!("-1 as u8 is :{}", -1i8 as u8);

    println!("1000 mod 256 is {}", 1000 % 256);

    println!("128 as a i16 is : {}", 128 as i16);
}
