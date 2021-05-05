fn main() {
    // instantiate the `Person` struct (or class)
    let p1 = Person {
        name: String::from("Aung Myat Moe"),
        age: 17,
        is_married: false,
    };
    println!("{:?}", p1);
    println!("Name is => {}", p1.name);
    // calling the concret method
    println!("{}", p1.get_info());

    // calling the static method
    println!("{}", Person::get_static_info());
}

// declare the class but structre in rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_married: bool,
}

// impl is to implement (or add) the methods of struct
impl Person {
    // &self => look like `this` keyword in other languages
    // concret methods must have `&self`
    // when we have `&self` as a param,yes! it is method
    fn get_info(&self) -> String {
        format!(
            "name: {},age: {},Husband: {}",
            &self.name, &self.age, &self.is_married
        )
    }

    // declare the static method
    fn get_static_info() -> String {
        format!("I am static method of rust struct (or class)")
    }
}
