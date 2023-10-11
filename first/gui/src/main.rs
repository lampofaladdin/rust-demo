use std::vec;

use gui::{Button, Draw, Screen};
struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectbox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 75,
                options: vec![
                    String::from("Yes"),
                    String::from("Mabye"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 75,
                label: String::from("fine"),
            }),
        ],
    };
    screen.run();
}
