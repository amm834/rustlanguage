#[allow(unused_variables)]
fn main() {
    let primes: Vec<i32> = Vec::new();
    let mut primes: Vec<i32> = vec![2, 4, 6, 8, 10];

    println!("{:?}", primes);
    println!("{}", primes[2]);

    primes[2] = 100;
    println!("{:?}", primes);

    const DEFAULT: bool = true;
    let nums = vec![DEFAULT; 10];
    println!("{:?}", nums);

    for num in nums.iter() {
        print!("{}, ", num);
    }
}
