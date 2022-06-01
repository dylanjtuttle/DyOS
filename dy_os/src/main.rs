#![no_std]  // Don't link the Rust standard library
#![no_main]  // Disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, world!";

// This function is the entry point, since the linker looks for a function named _start by default
#[no_mangle]  // Ensure that the Rust compiler actually makes the name of this function _start
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xB;
        }
    }

    loop {}
}

// To be run when the OS panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /* The ! return type tells the compiler that this function is never allowed to return - 
       it is the operating system after all, so if it crashes, that's it */
    loop {}
}
