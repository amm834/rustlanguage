fn main() {
    get_square(1000);
    get_cubic(1000);
}

fn get_square(limit : i32){
    let mut x = 0;
    while x*x < limit {
        println!("Square => {0} * {0} = {1}",x,x*x); 
        x += 1;
    }
}

fn get_cubic(limit : i32){
    let mut x = 0;
    loop {
        println!("Cubic => {0} * {0} = {1}",x,x*x*x);
        x += 1;
        if x*x*x > limit { break }
    }
}