trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String {
    fn dupl(&self) -> String {
        format!("{}", *self)
    }
}

impl Duplicateable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }

}

// static
/*fn duplicate<T:Duplicateable>(x:T){
println!("{}",x.dupl());
}*/

// dynamic
fn duplicate(x: &dyn Duplicateable) {
    println!("{}", x.dupl());

}

fn main() {
    let a = 20;
    let b = "Hello World ".to_string();
    /*duplicate(a);
    duplicate(b);*/
    duplicate(&a);
    duplicate(&b);
}