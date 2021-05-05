use std::io;

/// This crate is trying to achieve something.
/// This will do something.

fn main() {
//! # Main Function
//! ```rust
//! fn main(){
    
//! }
//```
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_)  => {
            println!("You said {}"
            ,input);
        },
        Err(e) => {
            println!("Something went wrong {}",e);
        }
    }
}
