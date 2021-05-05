
fn main() {
    let r = RustDev::new(String::from("John"));
    println!("{}", r.language());
    r.say_hello();
    println!("{:#?}",r);
}

#[derive(Debug)]
struct RustDev {
    name: String,
}

trait Developer {
    fn new(name: String) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello");
    }
}

impl Developer for RustDev {
    fn new(name: String) -> Self {
        RustDev { name }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("Hi {}", self.name);
    }
}
