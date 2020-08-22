// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;
extern crate rlibc;

static HELLO: &[u8] = b"Hello World!";

// don't mangle function name
#[no_mangle]
// entry point. Linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    // vga_buffer address location (raw pointer)
    let vga_buffer = 0xb8000 as *mut u8;

    // iterating over the bytes of the static HELLO byte string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // using offset to write the string byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // using offset to write color byte 0xb = light cyan
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb
        }
    }
    
    loop{}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}