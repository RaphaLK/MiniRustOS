#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minirustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minirustos::println;
// Override panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minirustos::test_panic_handler(info)
}

// static HELLO: &[u8] = b"Hello World";
// Disable name mangling -- C Calling convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    minirustos::init();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
    #[cfg(test)]
    test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
