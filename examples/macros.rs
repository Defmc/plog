use plog::{info, ok};
use std::{thread, time::Duration};

fn main() {
    let threads: Vec<_> = (0..=10)
        .map(|id| {
            info!("Creating thread {id}");
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1000));
                ok!("Thread {id} terminated");
            })
        })
        .collect();
        
    threads.into_iter().for_each(|thr| thr.join().unwrap());
}
