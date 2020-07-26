use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = [0u8; 4];
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buf).as_bytes().iter() {
                console_putchar(*code_point as usize)
            }
        }
        Ok(())
    }
}

pub fn print(arg: fmt::Arguments) {
    Stdout.write_fmt(arg).unwrap()
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
