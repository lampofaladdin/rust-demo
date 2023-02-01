fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f()
}

fn main() {
    println!("Hello, world!");
}
