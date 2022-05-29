use lazy_static::lazy_static;
use uart_16550::SerialPort;
use spin::Mutex;
use core::fmt;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Failed to write in the serial port");
}

#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => ($crate::serial::_print(format_args!($($args)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($args:tt)*) => ($crate::serial_print!(concat!($fmt, "\n") , $($args)*));
}
