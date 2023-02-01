#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(KinitOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use KinitOS::println;
mod cpu_exceptions;
// use crate::cpu_exceptions::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
} 

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    KinitOS::test_panic_handler(info)
} 

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Initializing boot process!");

    #[cfg(test)]
    test_main();
    
    loop{}
}
