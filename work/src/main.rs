fn main() {
    println!("请输入n阶 数列");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("读取内容失败");
    let index: i32 = index.trim().parse().expect("不是数字");
    if index < 2 {
        println!("结果:{}", index)
    } else {
        let mut looper = 0;
        let mut pre_val: i128 = 0;
        let mut c_val: i128 = 1;
        let mut result: i128 = 0;
        while looper < index {
            result = pre_val + c_val;
            pre_val = c_val;
            c_val = result;
            looper += 1;
            println!("结果：{}", result);
        }
    }
}
