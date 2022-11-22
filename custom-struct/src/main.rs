#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bootom_right: Point,
}

fn react_area(rectangle: Rectangle) -> f32 {
    let Point { x: left, y: top } = rectangle.top_left;
    let Point {
        x: right,
        y: bottom,
    } = rectangle.bootom_right;

    let width = right - left;

    let height = bottom - top;

    width * height
}

fn square(point: Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bootom_right: Point {
            x: &point.x + width,
            y: &point.y + width,
        },
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let petter = Person { name, age };
    println!("{:?}", petter);

    let point = Point { x: 1.1, y: 2.0 };
    println!("point coordinates:({},{})", point.x, point.y);

    let bottom_right = Point { x: 5.2, y: 4.0 };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bootom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 2.0);

    println!("pair contians {:?} and {:?}", pair.0, pair.1);

    let Pair(intger, decimal) = pair;

    println!("pair contaions {:?} and {:?}", intger, decimal);

    let area = react_area(_rectangle);

    println!("the rectangle area :{}", area);

    let square = square(Point { x: 1.0, y: 2.0 }, 2.0);
    println!("the square is {:?}", square);
}
