use bencher::*;
use plog::{info, ok};
use std::thread;

pub fn single_thread(b: &mut Bencher) {
    b.iter(|| ok!("Working"))
}

pub fn multi_thread(b: &mut Bencher) {
    let thrds = b.iter(|| thread::spawn(|| info!("Initialized")));
}

benchmark_group!(plog_benches, single_thread, multi_thread);
benchmark_main!(plog_benches);
