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

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("the value x {},the value y {}", x, y);
// }

// fn main() {
//     let x = 1;
//     let y = 2;
//     print_coordinates(&(x, y));
//     let b = Some(1);
//     let Some(x) = b;
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }

// fn main() {
//     let x = 99;

//     match x {
//         1 | 2 => println!("one or two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// fn main() {
//     let x = 5;

//     match x {
//         1..=5 => println!("one through five"),
//         _ => println!("something else"),
//     }
// }

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 0, y: 7, z: 8 };

    let Point { x: a, y: b, .. } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point { x: 0, y: 7 };

//     let Point { x, y } = p;
//     assert_eq!(0, x);
//     assert_eq!(7, y);
// }

// fn main() {
//     let p = Point { x: 0, y: 7 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point { x: 0, y } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }
