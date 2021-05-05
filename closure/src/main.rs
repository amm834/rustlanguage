fn main() {
    let a = |x: i32| x + 1;
    println!("{}", a(10));

    let b = |x: i32| -> i32 {
        let c = x + 2;
        c
    };
    println!("{}", b(20));

    let c = |x| x;
    c(123);
}
