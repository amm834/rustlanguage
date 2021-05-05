#[allow(unused_parens)]

fn main() {
    let uname ="amm834";
    let passwd ="123";
    
    if (uname == "amm834" && passwd == "123") {
    println!("Welcome user {}.",uname);
    }else{
        println!("Authenication Failed!");
    }
    // short-hand is more like ternary operator in other languages
    let result = if(5<10) { true } else { false };
    println!("{}",result);

}
