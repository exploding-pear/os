// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;

// don't mangle function name
#[no_mangle]
// entry point. Linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello world now sum nums: {}, {}", 14, 3.0/10.0);
    panic!("Some panic message");
    //loop{}
}

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}