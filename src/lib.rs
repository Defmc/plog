pub mod macros;

use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io;

#[cfg(feature = "persistent")]
pub mod persistent;

#[cfg(feature = "impls")]
mod impls {
    pub mod option_log;
    pub use option_log::*;
    pub mod result_log;
    pub use result_log::*;
}

#[cfg(test)]
pub mod test;

pub fn log<T: AsRef<str>>(color: Color, prefix: &str, msg: T) -> io::Result<()> {
    print_log(color, prefix, &msg)?;

    #[cfg(feature = "persistent")]
    if persistent::check_env() {
        persistent::write_log(prefix, &msg)?;
    }
    Ok(())
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
