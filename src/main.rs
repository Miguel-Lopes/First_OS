#![no_std] // Dont connect to the Rust standard library since we trying to make our own
#![no_main] // disable all entry points at the Rust-level 

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the function name so i dont create a 30 digit monstrosity
pub extern "C" fn _start() -> ! {
    // this function will be the entry point, since the linker will try to find a function
    // named `_start` by default

    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }


    loop {}
}

/// This function is called on panic (in case something bad happens).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}