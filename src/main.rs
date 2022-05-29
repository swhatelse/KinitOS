#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
use core::arch::asm;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    println!("{}", info);
    loop{}
} 

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    serial_println!("[test failure]");
    serial_println!("Error:\n {}", info);
    exit_qemu(QemuExitCode::Success);
    loop{}
} 

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Initializing boot process!");

    #[cfg(test)]
    test_main();
    
    loop{}
}

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

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn test() {
    assert_eq!(true, true);
}
