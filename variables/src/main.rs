fn main() {
    // let mut space = "你好";
    // space = space.len(&);
    // println!("the number is {}", space)
    // let x = 0.1;
    // let y = 0.2;
    // println!("x + y = {}", x + y);

    // // tup
    // let tup = (0.1, -12, 3);
    // let (a, b, c) = tup;
    // println!("a{} b{} c{}", a, b, c);
    // let aa = tup.0;
    // let bb = tup.1;
    // let cc = tup.2;

    // // 数组
    // let array = [1, 2, 3, 4, 5, 256, 9];
    // let array_0 = array[0];
    // let a = [1, 2, 3, 4, 5];
    // let mut index = String::new();
    // std::io::stdin().read_line(&mut index).expect("输入错误");
    // let index: usize = index.trim().parse().expect("不是数字");
    // let element = a[index];
    // println!("the index is {} value is {}", index, element)

    let b = another_funtion(4);
    println!("{}", b);
}

/**
 * 卧槽，这样子也行？
 */
fn another_funtion(x: i32) -> i32 {
    println!("this is another function {}", x);
    x
}
