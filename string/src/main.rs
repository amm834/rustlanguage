#[allow(unused_variables)]
fn main() {
    let name:&str = "RustLang";
    let dog:&'static str = "Bo Ni";
    let cat = String::new(); // create empty string object
    let mut cat = String::from("Noo Si");
    cat.push(' ');
    cat.push_str("the cat");
    println!("{}",cat);
    
    let new_cat = cat.replace("the","is my");
    println!("{}",new_cat);
    println!("{}",cat.len()); // length of string
}
