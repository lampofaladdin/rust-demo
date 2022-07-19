// fn main() {
//     let mut v = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);

//     let n = 0;

//     let b = match v.get(n) {
//         None => 0,
//         _ => v[n],
//     };
//     println!("{}", b);
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let does_not_exist = v.get(1);
//     match does_not_exist {
//         None => println!("None"),
//         Some(_) => println!("Some {} ", _),
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     v.push(6);

//     let first = &v[0];

//     println!("The first element is: {}", first);
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     for i in &mut v {
//         *i = *i * 10;
//         println!("{}", i);
//     }
//     println!("{:?}", v);
// }

// fn main() {
//     enum DataRow {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         DataRow::Int(1),
//         DataRow::Int(2),
//         DataRow::Float(2.5),
//         DataRow::Text(String::from("Hello")),
//     ];
//     for i in &row {
//         match i {
//             DataRow::Int(i) => println!("Int: {}", i),
//             DataRow::Float(f) => println!("Float: {}", f),
//             DataRow::Text(t) => println!("Text: {}", t),
//         }
//     }
// }

fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}
