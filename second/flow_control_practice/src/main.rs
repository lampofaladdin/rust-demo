// // 填空
// fn main() {
//     let n = 5;

//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// }

// // 修复错误
// fn main() {
//     let n = 50;

//     let big_n = if n < 10 && n > -10 {
//         println!(" 数字太小，先增加 10 倍再说");

//         10 * n
//     } else {
//         println!("数字太大，我们得让它减半");
//         n / 2
//     };

//     println!("{} -> {}", n, big_n);
// }

// // 修复错误，不要新增或删除代码行
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     }

//     println!("{:?}", numbers);
// }

// fn main() {
//     let a = [4, 3, 2, 1];

//     // 通过索引和值的方式迭代数组 `a`
//     for (i, v) in a.iter().enumerate() {
//         println!("第{}个元素是{}", i + 1, v);
//     }
// }

// 填空，让最后一行的  println! 工作 !
// fn main() {
//     // 一个计数值
//     let mut n = 1;

//     // 当条件为真时，不停的循环
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }

//         n = n + 1;
//     }

//     println!("n 的值是 {}, 循环结束", n);
// }

// // 填空，不要修改其它代码
// fn main() {
//     let mut n = 0;
//     for _i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//     }

//     assert_eq!(n, 66);
// }

// 填空，不要修改其它代码
// fn main() {
//     let mut n = 0;
//     for _i in 0..=100 {
//         if n != 66 {
//             n += 1;
//             continue;
//         }

//         break;
//     }

//     assert_eq!(n, 66);
// }

// // 填空，不要修改其它代码
// fn main() {
//     let mut count = 0u32;

//     println!("Let's count until infinity!");

//     // 无限循环
//     loop {
//         count += 1;

//         if count == 3 {
//             println!("three");

//             // 跳过当此循环的剩余代码
//             continue;
//         }

//         println!("{}", count);

//         if count == 5 {
//             println!("OK, that's enough");

//             break;
//         }
//     }

//     assert_eq!(count, 5);
// }

// // 填空
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);
// }
fn main() {
    let mut count = 0;
    'outer: loop {
        println!("outer");
        'inner1: loop {
            println!("inner1");
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also ok
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            println!("inner2");
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30)
}
