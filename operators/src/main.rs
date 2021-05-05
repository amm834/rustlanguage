fn main() {
    let a  = 1 + 2;
    let b = 10 - 5;
    let c = 2 * 5;
    let d = 10 / 2;
    let e  = 11 % 2;
    
    println!("a={},b={},c={},d={},e={}",a,b,c,d,e);
    
    println!("{}",a > b);
    
    println!("{}",a <= b && c >= d);
    
    println!("{}",1 << 2);
    println!("{}",10 ^ 20);
}
