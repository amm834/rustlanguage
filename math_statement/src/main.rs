use crate::Emoji::Crying;
use crate::Emoji::Heart;
use crate::Emoji::Smile;
fn main() {
    country(44);
    country(95);
    country(10);
    country(200);

    emoji(Heart);
    emoji(Smile);
    emoji(Crying);
}

fn country(code: u32) {
    let country = match code {
        44 => "UK",
        95 => "Myanmar",
        0..=99 => "Unknown",
        _ => "Invalid",
    };
    println!("Country is {}", country);
}

enum Emoji {
    Heart,
    Smile,
    Crying,
}

fn emoji(emoji: Emoji) {
    match emoji {
        Heart => println!("{}", "❤️"),
        Smile => println!("{}", "😊"),
        Crying => println!("{}", "😭"),
    }
}
