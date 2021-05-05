use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn start_thread(i:usize,tx:mpsc::Sender<usize>){
    thread::spawn(move || {
        println!("Starting - {}",i);
       // thread::sleep(Duration::from_secs(i as u64));
        println!("Sending - {}",i);
        tx.send(i).unwrap();
    });
}
const NUMS_THREAD:usize = 8000;

fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         tx.send(42).unwrap();
//     });
//     println!("recieved {}",rx.recv().unwrap());
 
let (tx,rx) = mpsc::channel();
for i in 0..NUMS_THREAD {
        start_thread(i,tx.clone());
}
for i in rx.iter().take(NUMS_THREAD) {
    println!("recieved {}",i);
}


}