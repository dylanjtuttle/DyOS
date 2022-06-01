#![no_std]  // Don't link the Rust standard library
#![no_main]  // Disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

// This function is the entry point, since the linker looks for a function named _start by default
#[no_mangle]  // Ensure that the Rust compiler actually makes the name of this function _start
pub extern "C" fn _start() -> ! {
    /* The ! return type tells the compiler that this function is never allowed to return - 
       it is the operating system after all, so if it crashes, that's it */
    
    // Our custom println! macro, not the built in one
    println!("Hello, world{}", "!");

    panic!("Some panic message");
}

// To be run when the OS panics
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panic message
    println!("{}", info);
    
    // Loop forever (this is still an OS, so we only want to exit when the computer is turned off)
    loop {}
}
