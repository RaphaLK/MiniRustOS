#![no_std]
#![no_main]
use core::panic::PanicInfo;

// Override panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Disable name mangling -- C Calling convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop{}
}