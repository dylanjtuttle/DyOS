#![no_std]  // Don't link the Rust standard library
#![no_main]  // Disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, world!";

// This function is the entry point, since the linker looks for a function named _start by default
#[no_mangle]  // Ensure that the Rust compiler actually makes the name of this function _start
pub extern "C" fn _start() -> ! {
    // Cast 0xb8000 into a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // Iterate through the bytes in the "Hello, world!" byte string
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Write the current byte to the screen
            *vga_buffer.offset(i as isize * 2) = byte;
            // Set the color of this byte to cyan (0xB)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xB;
        }
    }

    // Loop forever (this is still an OS, so we only want to exit when the computer is turned off)
    loop {}
}

// To be run when the OS panics
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /* The ! return type tells the compiler that this function is never allowed to return - 
       it is the operating system after all, so if it crashes, that's it */
    loop {}
}
