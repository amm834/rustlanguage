fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("Sum = {}", a.sum());
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += i;
        }
        sum
    }
}
