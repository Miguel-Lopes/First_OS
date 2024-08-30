#![no_std] // Dont connect to the Rust standard library since we trying to make our own
#![no_main] // disable all entry points at the Rust-level 

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the function name so i dont create a 30 digit monstrosity
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    loop {}
}


/// This function is called on panic (in case something bad happens).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}