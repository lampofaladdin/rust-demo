// fn main() {
//     let user1 = build_user(String::from("alad1din"), String::from("&1295106869@qq.com"));
//     let user2 = User {
//         username: String::from("light"),
//         ..user1
//     };

//     println!("{}", user1.username)
// }

// fn build_user(username: String, email: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let color1 = Color(0, -1, 0);
//     let point1 = Point(0, -1, 0);
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let subject = AlawaysEqual;
// }
// struct AlawaysEqual;

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
// }
// 结构体使用字符串面量，会导致报错。生命周期？
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

/**
 * 计算面积，普通方式
 */
// fn main() {
//     let width = 30;
//     let height = 30;
//     println!("the height * width = {}", area(width, height))
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/**
 * 计算面积，使用元组方式
 */
// fn main() {
//     let area_param = (30, 30);
//     println!("the height * width = {}", area(area_param))
// }

// fn area(area_param: (u32, u32)) -> u32 {
//     return area_param.1 * area_param.1;
// }

/**
 * 计算面积，结构体方式
 */
// fn main() {
//     let rect = Rectangle {
//         width: 32,
//         height: 32,
//     };
//     println!("the height * width = {}", area(&rect));
//     println!("rect 1 {:#?} 2", rect);
// }

// // 不加 & 会导致后续内容无法调用rect 这个结构体
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// /**
//  * debug方式有点日怪
//  */
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect = Rectangle {
//         width: dbg!(30 * scale),
//         height: 32,
//     };
//     dbg!(&rect);
// }

// /**
//  * debug方式有点日怪
//  */
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&mut self, b: u32) -> u32 {
        self.width * self.height * b;
        self.width = b;
        self.width * self.height * b
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

/**
 * 可以定义多个 impl
 */
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect3 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("1 can hold 2 = {:#?}", rect1.can_hold(&rect2));
    println!("1 can hold 3 = {:#?}", rect1.can_hold(&rect3));
    let square = Rectangle::square(20);
    dbg!(square);
}
