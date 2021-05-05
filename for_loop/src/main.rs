fn main() {
    // loop through the range
    for i in 1..=10 {
        println!("{} * {} = {}",i,i,(i*i));
    }
    
    let pets = ["cat","dog","fox","bear"];
    // iterating the array
    for pet in pets.iter() {
        if pet == &"fox" {
            println!("I don't like {}.",pet);
            continue
        }
        if pet == &"bear" {
            println!("{} is not pet.",pet);
            break
        }
        println!("I love {}",pet);
    }
    
    // iterating the range with position value
    for (pos,i) in (1..10).enumerate() {
        println!("{} * {} = {}",(pos + 1),i,((pos + 1) * i));
    }
}
