// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// testing related info
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;

// don't mangle function name
#[no_mangle]
// entry point. Linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    #[cfg(test)]
    test_main();

    loop{}
}

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}