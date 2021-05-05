static mut R: i32 = 4;
fn main() {
    // scope
    {
        let a = 10;
        println!("{}", a);
    }

    // changing the mutable static variable will be unsafe (it raises of memory and other memory violation will happen)
    unsafe {
        println!("{}", R);
        R = 10;
        println!("{}", R);
    }

    unsafe {
        println!("{}", R);
    }
}
