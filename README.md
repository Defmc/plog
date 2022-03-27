# plog
A pretty logger written for Rust programs.

A library to log applications just using macros, includes initially five levels: `debug!`, `info!`, `ok!`, `warn!` and `error!`, each one distributed as a macro:
```rust
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
```

## features

Due to the focus on optimization, every functionality from the lib (except log macros) is offered by features. Plog contains the following features:
| feature | description | additional dependence | why |
| ------- | ----------- | --------------------- | --- |
| `datetime` | Include date and time on the log, formatted as `%Y-%M-%D %HH:%MM:%SS` | `chrono` | To format system time |
| `persistent` | Add a persistent log handled by `LOG_FILEPATH` environment variable, saving each log on the pointed file | `parking_lot` | `const_mutex` for stable Rust and optimizations |
| `context`| Include the line and file that has required the log | none except `std::line!` and `std::file!` | To get information about in compile-time |

## plans
- [] Split date and time into two features
- [] Include logging methods to `Option` and `Result`
- [] Optimize code readability
- [] Include benchmarks
- [] Create documentation
