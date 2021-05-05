fn main() {
    println!("{}", "Hello World!");
    println!(
        "{0} has a {1} and {0}  has a {2} .",
        "Aung Myat Moe", "cat", "dog"
    );
    println!("Fullname : {fullname} ", fullname = "Aung Myat Moe");
    println!("Binary = {:b}, Hex = {:x}, Octal = {:o}", 50, 50, 50);
    println!("{:#?}", [1, 2, 3]);
    
}
