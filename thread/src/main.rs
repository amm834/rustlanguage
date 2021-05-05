use std::thread;
use std::time::Duration;

fn main() {

    let mut threads = vec![];
    
    for i in 1..10 {

        let mut th = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i* 1000));
            println!("New thread - {}", i);
        });
        
        threads.push(th);

    }
    
    for t in threads {
    // waiting threads
        t.join();
    }

    println!("Main thread");
}