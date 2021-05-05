#[allow(dead_code)]
mod search;

fn main() {
    search::search_file("hello.txt");
    clean::clean();
    clean::format::format();
}

mod clean {
    // internal module
    pub fn clean() {
        println!("Cleaning HDD");
    }

    // nesting the modules
    pub mod format {
        pub fn format() {
            println!("Format SDD");
        }
    }
}

