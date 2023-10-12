fn main() {
    // let mut x = 5;
    // println!("The value of x {x}");
    // x = 6;
    // println!("The value of x {x}");

    // 子域内的遮蔽不会影响到父级作用域
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x {x}");
    // }
    // println!("The value of x {x}");

    // let sum = 5 + 10;
    // let diffrence = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quotient = 56.7 / 32.2;

    // 整数与证书相除得到的不是小数
    // let truncated = -5.0 / 3.0;

    // println!("The value of quotient {quotient}");

    // let remainder = 43 % 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
