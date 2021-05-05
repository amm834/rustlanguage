fn main() {
    let dog = Dog {
        species : "Retriever"
    };
    let cat = Cat {
        color : "White & Soft Black"
    };
    bark_it(dog);
}

trait Bark{
    fn bark(&self) -> String;
}

struct Dog {
    species : &'static str
}

struct Cat {
    color : &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("Barking")
    }
}
fn bark_it<T:Bark>(b:T){
    println!("{}",b.bark());
}