fn main() {
    let primes = [2, 4, 6, 8, 10];
    let doubles: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);
    println!("{}", primes[3]);
    const DEFAULT: i32 = 0;

    let mut nums = [DEFAULT; 15];
    println!("{:?}", nums);

    nums[1] = 1;
    println!("{:?}", nums);

    // loop through the array
    for num in nums.iter() {
        print!("{}", num);
    }
}
