pub mod macros;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
};
use std::io;

pub fn log<T: AsRef<str>>(color: Color, prefix: &str, msg: T) -> io::Result<()> {
    execute!(
        io::stderr(),
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
