fn main() {
    let nums = [1, 2, 3, 4, 5];
    let slices = &nums[1..3];
    println!("{:?}", slices);

    let mut colors = ["red", "green", "blue"];
    
    println!("{:?}", colors);
    update_slices(&mut colors[0..=2]);
    println!("{:?}", colors);
}

fn update_slices(slice: &mut [&str]) {
    slice[0] = "violet";
    slice[1] = "pink";
}
