enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

#[derive(Debug)]
enum VeryLongEnumToDoSomeThingWithNumber {
    Add,
    Subtract,
}

type Opeartion = VeryLongEnumToDoSomeThingWithNumber;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click { x, y } => println!("clied ({},{})", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('c');

    let pasted = WebEvent::Paste(String::from("it is paste string"));

    let clicked = WebEvent::Click { x: 1, y: 2 };

    let loaded = WebEvent::PageLoad;

    let unloaded = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(loaded);
    inspect(unloaded);

    let add = Opeartion::Add;

    println!("add {:?}", add);
}
