fn main() {
    let mut person = ("John", 17, false);
    println!("{:?}", person);
    println!("{}", person.0);
    person.0 = "Aung Myat Moe";
    println!("{:?}", person);

    // tuple desctruction
    // it look like object & array desctruction in JavaScript
    let (name, age, is_married) = person;
    println!("name {},age {},isMarried {}", name, age, is_married);
}
