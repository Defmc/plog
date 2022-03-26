pub mod macros;

use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use parking_lot::{const_mutex, Mutex};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    env,
    fs::File,
    io::{self, Write},
};

static LOG_FILE: Mutex<Option<File>> = const_mutex(None);
static USE_LOG_FILE: AtomicBool = AtomicBool::new(false);
static CHECKED_ENV: AtomicBool = AtomicBool::new(false);

pub type LogResult<T> = Result<T, Box<dyn Error>>;

pub fn log<T: AsRef<str>>(color: Color, prefix: &str, msg: T) -> io::Result<()> {
    print_log(color, prefix, &msg)?;
    if check_env() {
        write_log(prefix, &msg)?;
    }
    println!("{}", check_env());
    println!("{LOG_FILE:?}");
    Ok(())
}

#[inline(always)]
fn check_env() -> bool {
    if !CHECKED_ENV.load(Ordering::SeqCst) {
        CHECKED_ENV.store(true, Ordering::SeqCst);
        USE_LOG_FILE.store(init_log_file().is_ok(), Ordering::SeqCst);
    }
    USE_LOG_FILE.load(Ordering::Acquire)
}

fn init_log_file() -> LogResult<()> {
    let path = env::var("LOG_FILEPATH")?;
    let file = File::create(path)?;
    let mut log_file = LOG_FILE.lock();
    *log_file = Some(file);
    Ok(())
}
fn write_log<T: AsRef<str>>(prefix: &str, msg: &T) -> io::Result<()> {
    let logfile = &mut LOG_FILE.lock();
    writeln!(
        &mut logfile.as_mut().unwrap(),
        "[{prefix}]: {}",
        msg.as_ref()
    )
}

fn print_log<T: AsRef<str>>(color: Color, prefix: &str, msg: &T) -> io::Result<()> {
    execute!(
        io::stderr().lock(),
        Print("["),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(color),
        Print(prefix),
        ResetColor,
        Print("]: "),
        Print(msg.as_ref()),
        Print('\n')
    )
}
