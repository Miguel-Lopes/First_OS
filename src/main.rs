#![no_std] // Dont connect to the Rust standard library since we trying to make our own
#![no_main] // disable all entry points at the Rust-level 

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#![reexport_test_harness_main = "test_main"]

#[no_mangle] // don't mangle the function name so i dont create a 30 digit monstrosity
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
We set 


/// This function is called on panic (in case something bad happens).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
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