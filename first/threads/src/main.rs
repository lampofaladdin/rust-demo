// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handler = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     handler.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || {
        println!("here is a vector:{:?}", v);
    });
    handler.join().unwrap();
}
