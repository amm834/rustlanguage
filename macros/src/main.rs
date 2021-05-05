macro_rules! my_macro {
    () => {
        println!("Hello")
    };
}

macro_rules! name {
    ($name: expr) => {
        println!("{}", $name)
    };
}

macro_rules! call_names {
    ($($names: expr), *) => {
        $(
            println!("Hey {}", $names);
        ) *
    }
}

macro_rules! xy {
    (x => $e: expr) => {
        println!("x is {}", $e)
    };
    (y => $e: expr) => {
        println!("y is {}", $e)
    };
    (x => $e: expr, y => $f: expr) => {
        println!("x is {} and y is {}", $e, $f)
    };
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("Function {:?} was called.", stringify!($fn_name));
        }
    };
}

fn main() {
    my_macro!();
    name!("Noosi");

    call_names!("Alex", "John");

    xy!(x => 10);
    xy!(y => 2* 6);
    xy!(x => 10, y => 20);
    build_fn!(hello);
    hello();
}
