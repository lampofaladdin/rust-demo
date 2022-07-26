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

// use std::collections::HashMap;

// /**
//  * 平均数，需要转换成f64
//  */
// fn get_average(num_array: &Vec<i32>) -> f64 {
//     let sum: i32 = num_array.iter().sum();
//     let f_sum = sum as f64;
//     let average = f_sum / num_array.len() as f64;
//     println!("{}", average);
//     f_sum
// }

// /**
//  * sort方法会借用原数组，无法在下一个方法使用
//  */
// fn get_mid(mut num_array: Vec<i32>) -> i32 {
//     num_array.sort_by(|a, b| a.cmp(b));
//     let mid = num_array.iter().len() / 2;
//     println!("{}", num_array[mid]);
//     num_array[mid]
// }

// /**
//  * 重复书最多的数字
//  */
// fn get_mod(num_array: Vec<i32>) -> i32 {
//     let mut counter_map = HashMap::new();
//     for num in num_array.iter() {
//         let count = counter_map.entry(num).or_insert(0);
//         *count += 1;
//     }
//     let mut max_repeat = 0;
//     let mut current_num = 0;
//     for (key, val) in counter_map.into_iter() {
//         if val > max_repeat {
//             max_repeat = val;
//             current_num = *key;
//         }
//     }
//     current_num
// }

// fn main() {
//     let num_array = vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 5, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2];
//     let average = get_average(&num_array);
//     // let mid = get_mid(num_array);
//     let mod_num = get_mod(num_array);
//     println!("{}", mod_num)
// }

/**
 * 完成  Pig Latin
 */
// fn main() {
//     let on_words: [u8; 5] = [97, 111, 101, 105, 117];

//     println!("请输入单词");
//     let mut str = String::new();
//     loop {
//         std::io::stdin().read_line(&mut str).expect("请输入单词");
//         str = str.trim().to_string();
//         let first_word = str.bytes().next();
//         match first_word {
//             Some(first_word) => {
//                 if on_words.contains(&first_word) {
//                     str.push_str("-hay");
//                 } else {
//                     let b = first_word.escape_ascii().to_string();
//                     str.push_str("-");
//                     str.push_str(&b);
//                     str.push_str("ay")
//                 }
//                 break;
//             }
//             None => {
//                 println!("输入单词错误，请重新输入");
//                 continue;
//             }
//         }
//     }
//     println!("{}", "结果是：");
//     println!("{}", str);
// }
use std::collections::HashMap;

struct Company {
    oragins: HashMap<String, Vec<String>>,
}
impl Company {
    pub fn add_people(&mut self, department: String, name: String) {
        self.oragins
            .entry(department)
            .or_insert(Vec::new())
            .push(name);
    }
    pub fn get_department_people(&mut self, department: &String) -> Vec<String> {
        let result = match self.oragins.get(department) {
            Some(peoples) => peoples.clone(),
            None => Vec::new(),
        };
        result
    }
    pub fn get_all_people(&mut self) -> Vec<&String> {
        let mut peoples = Vec::new();
        for oragin in self.oragins.iter() {
            for people in oragin.1 {
                peoples.push(people)
            }
        }
        peoples
    }
}

fn main() {
    let mut company = Company {
        oragins: HashMap::new(),
    };
    company.add_people("开发部".to_string(), "张三".to_string());
    company.add_people("开发部".to_string(), "李四".to_string());
    company.add_people("销售部".to_string(), "王五".to_string());
    let department_people = company.get_department_people(&"开发部".to_string());               
    let peoples = company.get_all_people();
    println!("{:?}", department_people);
    println!("{:?}", peoples);
}
