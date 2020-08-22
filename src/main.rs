// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// don't mangle function name
#[no_mangle]
// entry point. Linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    loop{}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}