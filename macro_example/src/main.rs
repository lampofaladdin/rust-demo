use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("hello macro! my name is Pancakes");
    }
}

fn main() {
    Pancakes::hello_macro()
}
