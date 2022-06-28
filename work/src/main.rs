use std::time::SystemTime;

fn main() {
    println!("请输入n阶 数列");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("读取内容失败");
    let index: i32 = index.trim().parse().expect("不是数字");
    let sys_time1 = SystemTime::now();
    feb(index);
    let sys_time2 = SystemTime::now();
    let difference = sys_time2
        .duration_since(sys_time1)
        .expect("Clock may have gone backwards"); //expect()方法在Ok时返回T值，在Err时panic
    println!("{:?}", difference)
}

fn feb(n: i32) -> i128 {
    if n < 2 {
        return n.into();
    } else {
        let mut looper = 0;
        let mut pre_val: i128 = 0;
        let mut c_val: i128 = 1;
        let mut result: i128 = 0;
        while looper < n {
            result = pre_val + c_val;
            pre_val = c_val;
            c_val = result;
            looper += 1;
        }
        return result;
    }
}
