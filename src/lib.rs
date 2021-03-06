#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod serial;
use core::panic::PanicInfo;
use core::arch::asm;

/*************************************************
 *
 *                Panic Handler
 *
 ************************************************/
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[test failure]");
    serial_println!("Error:\n {}", info);
    exit_qemu(QemuExitCode::Failure);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
} 

/*************************************************
 *
 *                Exit Qemu
 *
 ************************************************/
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode){
    unsafe {
        asm!("out 0xf4, eax", in("eax") exit_code as u32);
    }
}

/********************************************************
 *
 *                    Test framework
 *
 *******************************************************/
pub trait Testable {
    fn run(&self)-> ();
}

impl<T> Testable for T
where T: Fn()
{
    fn run(&self){
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{}
}
