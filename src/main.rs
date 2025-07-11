#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;

// Override panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// static HELLO: &[u8] = b"Hello World";

// Disable name mangling -- C Calling convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();

    loop {}
}
