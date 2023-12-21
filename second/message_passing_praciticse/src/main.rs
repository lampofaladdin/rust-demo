// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// use std::sync::Mutex;

// fn main() {
//     // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
//     let m = Mutex::new(5);

//     // 获取锁，然后deref为`m`的引用
//     // lock返回的是Result
//     let mut num = m.lock().unwrap();
//     *num = 6;
//     // 锁自动被drop

//     println!("m = {:?}", m);
// }

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     let mut num = m.lock().unwrap();
//     *num = 6;
//     // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
//     drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
//     let mut num1 = m.lock().unwrap();
//     *num1 = 7;
//     // drop(num1); // 手动 drop num1 ，观察打印结果的不同

//     println!("m = {:?}", m);
// }

// use std::sync::Mutex;
// use std::thread;

// use std::sync::Arc;

// fn main() {
//     // 通过`Rc`实现`Mutex`的多所有权
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     // 等待所有子线程完成
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // 输出最终的计数结果
//     println!("Result: {}", *counter.lock().unwrap());
// }

// use std::thread::sleep;
// use std::time::Duration;
// use std::{
//     sync::{Mutex, MutexGuard},
//     thread,
// };

// use lazy_static::lazy_static;
// lazy_static! {
//     static ref MUTEX1: Mutex<i64> = Mutex::new(0);
//     static ref MUTEX2: Mutex<i64> = Mutex::new(0);
// }

// fn main() {
//     // 存放子线程的句柄
//     let mut children = vec![];
//     for i_thread in 0..2 {
//         children.push(thread::spawn(move || {
//             for _ in 0..1 {
//                 // 线程1
//                 if i_thread % 2 == 0 {
//                     // 锁住MUTEX1
//                     let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

//                     println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

//                     // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
//                     sleep(Duration::from_millis(10));

//                     // 去锁MUTEX2
//                     let guard = MUTEX2.lock().unwrap();
//                 // 线程2
//                 } else {
//                     // 锁住MUTEX2
//                     let _guard = MUTEX2.lock().unwrap();

//                     println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

//                     let _guard = MUTEX1.lock().unwrap();
//                 }
//             }
//         }));
//     }

//     // 等子线程完成
//     for child in children {
//         let _ = child.join();
//     }

//     println!("死锁没有发生");
// }

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

struct Counter {
    count: u64,
}

fn main() {
    let n = Mutex::new(Counter { count: 0 });

    n.lock().unwrap().count += 1;

    let n = AtomicU64::new(0);

    n.fetch_add(0, Ordering::Relaxed);
}
