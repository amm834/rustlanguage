#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person,
}

impl Person {
    fn get_name(&self) -> &String
    // fn get_name<'l>(&'l self) -> &'l String 
    {
        &self.name
    }
}

fn main() {
    println!("{}", get_str());
    let p = Person {
        name: "John".to_string(),
    };
    let d = Dog {
        name: "Stubby".to_string(),
        owner: &p,
    };

    let a: &String;
    {
        let p2 = Person {
            name: "Mary".to_string(),
        };
        // p2 will drop when local area is exit
        // a = p2.get_name();
        // but global scope will not exit so we can reassign if the structyre of method is equal
        a = p.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello"
}
