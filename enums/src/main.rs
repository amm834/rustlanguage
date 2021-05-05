use crate::Colors::RED;
use crate::Person::Name;

fn main() {
    let red_color = Colors::RED;
    println!("{:?}", red_color);
    let red_color = RED;
    println!("{:?}", red_color);
    let person_name = Name(String::from("Aung Myat Moe"));
    println!("{:?}", person_name);
}

#[derive(Debug)]
enum Colors {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Age(u32),
}
