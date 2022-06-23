
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数字");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("输入你猜的数字");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取文本");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入1-100 之间的数字");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("刚刚好");
                break;
            }
        };
    }
}
