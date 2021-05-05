use std::rc::Rc;

struct Car {
    brand : Rc<String>,
}

impl Car {
    fn new(brand:Rc<String>) -> Self {
        Car {
            brand
        }
    }
    fn drive(&self){
        println!("I'm driving {}",self.brand);
    }
}

fn main() {
    let brand = Rc::new(String::from("BMW"));
    println!("pointers {}",Rc::strong_count(&brand));
    
    {
        let c = Car::new(brand.clone());
        c.drive();
        println!("pointers {}",Rc::strong_count(&brand));

    }
    
    println!("My car is a {}", brand);
    println!("pointers {}",Rc::strong_count(&brand));

    
}
