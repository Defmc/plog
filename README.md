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
| `local_date` | Include local date on the log, formatted as `%Y-%M-%D` | `chrono` (branch `time-0.3`) | Get local date information and formatting |
| `local_time` | Similar to `local_date`, but logs the time with `%H:%M:%S` | `chrono` (branch `time-0.3`) | Get local time information and formatting |
| `context`| Include the line and file that has required the log | none | |
| `colored` | Use escape sequences to print stylized text | `crossterm` | Crossplatform escape sequences parser |
| `persistent` | Add a persistent log handled by `LOG_FILEPATH` environment variable, saving each log on the pointed file | `parking_lot` (until 0.2.14) | `const_mutex` for stable Rust and optimizations |
| `impls` | Implements `log` and `show_<variant>` for `Option<T>` and `Result<T, U>`| none | |

## plans
- [X] Split date and time into two features
- [X] Include logging methods to `Option` and `Result`
- [ ] Optimize code readability
- [X] Include benchmarks
- [X] Create documentation
