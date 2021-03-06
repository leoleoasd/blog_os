

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (vga_print!($($arg)*); serial_print!($($arg)*););
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
