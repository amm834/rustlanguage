use crate::Colors::Green;
use crate::Colors::Red;

fn main() {
    let point: Point<i32> = Point { x: 1, y: 2 };
    println!("{:?}", point);
    let red = Red("#ff0000");
    let green = Green("#00ff16");
    println!("{:?}", red);
    println!("{:?}", green);

    let p: Person<String, u32> = Person {
        name: String::from("Aung Myat Moe"),
        age: 17,
    };
    println!("{:?}", p);
}

// generic struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Green(T),
}

#[derive(Debug)]
struct Person<S, I> {
    name: S,
    age: I,
}
