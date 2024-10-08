#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(first_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    first_os::init();

    //Invokes a breakpoint exception
    x86_64::instructions::interrupts::int3();


    #[cfg(test)]
    test_main();

    println!("Its a christmas miracle, it did not crash!");
    first_os::hlt_loop();  
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    first_os::hlt_loop(); 
}

///This function is called on panic in case it is a test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    first_os::test_panic_handler(info)
}