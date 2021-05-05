fn main() {
    for i in 0..20 {
        println!("I have {} oranges", get_orange(i));
    }

    let value = (1, 1);
    // tuples matching
    match value {
        (0, 0) => println!("origin"),
        (x, 0) => println!("At x axis ({},0)", x),
        (0, y) => println!("At x axis (0,{})", y),
        (x, y) => println!("({},{})", x, y),
    };
}

fn get_orange(value: i32) -> &'static str {
    return match value {
        0 => "no",
        1 | 2 => "a little",
        0..=10 => "a few",
        _ if value % 2 == 0 => "an even of orange",
        _ => "lot of",
    };
}

