fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    };

    let closeure_annotated = |i| -> i32 { i + 1 };
    let closeure_inferred = |i| i + 1;

    let i = 1;

    println!("function {}", function(i));
    println!("closeure_annotated {}", closeure_annotated(i));
    println!("closeure_inferred {}", closeure_inferred(i));

    let one = || 1;
    println!("closeure return one :{}", one())
}
