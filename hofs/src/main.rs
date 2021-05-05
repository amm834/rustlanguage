fn main() {
    get_square(|x| x * x, 10);

    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let sq = i * i;
        if sq > limit {
            break;
        } else {
            if is_even(sq) {
                sum += sq;
            }
        }
    }
    println!("Without higher order function => {}", sum);

    //With HOFs
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("The sum using HOFs is {}", sum2);
}

fn get_square(f: fn(x: i32) -> i32, x: i32) {
    println!("Area of square is {}", f(x));
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}
