#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;

// Override panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// static HELLO: &[u8] = b"Hello World";

// Disable name mangling -- C Calling convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Help!!");
    loop {}
}
