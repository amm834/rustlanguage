fn main() {
    // primitive types can copy the value and ownership
    let a = 10;
    let b = a;
    println!("{}", a);
    println!("{}", b);

    // complex types move ownership and value to new variable
    // and moved variable no longer access to its value
    let v = vec![1, 2, 3, 4, 5];

    /*  let x = v;
    println!("{:?}",x);
    println!("{:?}",v);
    */


    let c = |v: Vec<i32>| -> Vec<i32> { v };

    let v = c(v);
    println!("{:?}", v);
}
