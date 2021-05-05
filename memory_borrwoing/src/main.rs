fn main() {
    let mut a = 5;

    {
        let b = &mut a;
        println!("{}", *b);
        a += 3;
    }
    println!("{}", a);
    
    let mut v = vec![1,2,3];
    /*for i in &mut v{
        println!("{}",i);
        v.push(20);
    }*/

}