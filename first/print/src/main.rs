fn main() {
    println!("{} day", 31);

    println!("{0} this is {1},{1},this is {0}", "alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 3);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print...", Structure(3));

    let pi = 3.1415926;

    println!("Pi is roughly {0:.3}", pi)
}
