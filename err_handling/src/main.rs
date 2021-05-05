use std::fs::File;

fn main() {
   // let f = File::open("noosi.png").unwrap();
    let f = File::open("noosi.png").expect("Failed to open file");
}
