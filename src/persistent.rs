/// Persistent log mode, enabled by `persistent` feature
/// Writes to a file (defined with `LOG_FILEPATH` environment variable)
/// Thread-safe, requires `parking_lot` to compile
use parking_lot::{const_mutex, Mutex};
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

/// Output file, defined in the first `plog::log` call
static LOG_FILE: Mutex<Option<File>> = const_mutex(None);

/// A `LOG_FILE.is_some()` wrapper, used to not freeze the `LOG_FILE`
static USE_LOG_FILE: AtomicBool = AtomicBool::new(false);

/// Used to define if the environment was already checked
static CHECKED_ENV: AtomicBool = AtomicBool::new(false);

/// Checks the environment
/// If a `LOG_FILE` was defined, will create a file and set `USE_LOG_FILE` as true
pub fn check_env() -> bool {
    if !CHECKED_ENV.load(Ordering::SeqCst) {
        CHECKED_ENV.store(true, Ordering::SeqCst);
        USE_LOG_FILE.store(init_log_file().is_ok(), Ordering::SeqCst);
    }
    USE_LOG_FILE.load(Ordering::Acquire)
}

/// Creates the log file. If already exists something with the same filepath, just append it
fn init_log_file() -> Result<(), Box<dyn Error>> {
    let path = env::var("LOG_FILEPATH")?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    let mut log_file = LOG_FILE.lock();
    *log_file = Some(file);
    Ok(())
}

pub fn write_log<T: AsRef<str>>(prefix: &str, msg: &T) -> io::Result<()> {
    let logfile = &mut LOG_FILE.lock();
    writeln!(
        &mut logfile.as_mut().unwrap(),
        "[{prefix}]: {}",
        msg.as_ref()
    )
}
