fn main() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("color:{}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 1;

    let mut inc = || {
        count += 1;
        println!("count:{}", count);
    };

    inc();
    inc();

    let _reborrow = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable:{:?}", movable);
        mem::drop(movable);
    };
    consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("contains 1:{}", contains(&1));
    println!("contains 2:{}", contains(&2));
    println!("contains 5:{}", contains(&5));
}
