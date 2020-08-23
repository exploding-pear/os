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
mod serial;
mod vga_buffer;

// don't mangle function name
#[no_mangle]
// entry point. Linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    #[cfg(test)]
    test_main();

    loop {}
}

// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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
fn trivial_assertion() {
    assert_eq!(1, 1);
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(), // T must implement the Fn() trait i.e. be a function
{
    fn run(&self) {
        // printing the function name (for functions, the type is the function name)
        serial_print!("{}...\t", core::any::type_name::<T>());
        // calling test function
        self();
        // printing ok
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
