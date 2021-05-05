use std::fs::File;
use std::io;
use std::io::Read;

fn get_uname_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/example.txt")?;
    let mut contents = String::new();
    let r = f.read_to_string(&mut contents)?;
    Ok(r.to_string())
}

fn main() {
    println!("{:?}", get_uname_from_file());
}
