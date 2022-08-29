// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();

//     if let Some(color) = favorite_color {
//         println!("use your favorite color,{},as the background ", color);
//     } else if is_tuesday {
//         println!("tuesday is green day ");
//     } else if let Ok(age) = age {
//         if (age) > 30 {
//             println!("use the purple color as the background");
//         } else {
//             println!("useing the orange color as the background");
//         }
//     } else {
//         println!("useing blue color as the background");
//     }
// }

// fn main() {
//     let mut stack: Vec<i32> = vec![];
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     while let Some(top) = stack.pop() {
//         println!("the top is {}", top);
//     }
// }

// fn main() {
//     let v = vec!["a", "b", "c"];
//     for (index, value) in v.iter().enumerate() {
//         println!("the index is {},the value is {}", index, value);
//     }
// }

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("the value x {},the value y {}", x, y);
}

fn main() {
    let x = 1;
    let y = 2;
    print_coordinates(&(x, y));
}
