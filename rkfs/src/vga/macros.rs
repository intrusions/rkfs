#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = $crate::vga::writer::WRITER
            .lock()
            .write_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}
