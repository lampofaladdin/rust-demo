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
//     }  b
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{}-{}-{}", s1, s2, s3);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut socres = HashMap::new();
//     socres.insert(String::from("hello"), 10);
//     socres.insert(String::from("word"), 20);
//     println!("{:?}", socres)
// }

// use std::collections::HashMap;
// fn main() {
//     let teams = vec![String::from("hello"), String::from("word")];
//     let initial_scores = vec![10, 50];
//     let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
//     println!("{:?}", scores)
// }

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     // scores.entry(String::from("Yellow")).or_insert(50);
//     // scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }

// fn main() {
//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
//     println!("{:?}", map)
// }

fn get_average(num_array: Vec<i32>) -> i32 {
    let sum: i32 = num_array.iter().sum();
    let average = sum.;
    println!("{}", average);
    average
}

fn main() {
    let num_array = vec![1, 2, 3, 4, 5];
    get_average(num_array);
}
