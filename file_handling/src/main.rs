use std::fs::{File,OpenOptions,remove_file};
use std::io::Write;
use std::io::Read;

fn main() {
    /*let mut file = File::create("src/example.txt").expect("Failed to create file");
    file.write_all("Hello!\n".as_bytes()).expect("Failed fo write file");*/
    
    /*let mut file = OpenOptions::new().append(true)
    .open("src/example.txt")
    .expect("Failed to open file");
    file.write_all("New content\n".as_bytes()).expect("Failed fo write file");*/
    
    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
    
    remove_file("src/example.txt").expect("Failed to delete");

}
