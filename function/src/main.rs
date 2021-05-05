#[allow(unused_variables)]

fn main() {
    for i in 1..6 {
        say_hi();
    }

    let mut name = "John";

    get_name(&mut name); // passing through by reference
    println!("{}", name);

    return_type(name);
}

fn say_hi() {
    println!("Hi");
}

fn get_name(name: &mut &str) {
    println!("Your name is {}.", name);
    *name = "Aung Myat Moe";
}

fn return_type(name: &str) -> String {
    format!("Hello {}", name)
}
