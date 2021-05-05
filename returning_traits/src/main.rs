// traits in rust are more like interface and abstract class in other languages
trait Animal {
    fn make_noise(&self) -> &'static str;
}

struct Dog {

}

struct Cat {
    
}

impl Animal for Dog {
   fn make_noise(&self) -> &'static str {
        "woff"
    }
}

impl Animal for Cat {
   fn make_noise(&self) -> &'static str {
        "meow"
    }
}

// returning the type of trait (or interface) must implement with `dyn` to know size of trait in run time
fn  says(rand_num:f64) -> Box<dyn Animal> {
    if rand_num >= 1.0 {
        Box::new(Cat {})
    }else {
        Box::new(Dog {})
    }
}

fn main() {
    println!("{}",says(1.0).make_noise());
    println!("{}",says(0.5).make_noise());
}
