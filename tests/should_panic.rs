#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minirustos::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");
  exit_qemu(
    QemuExitCode::Failed
  );
  loop {}
}

fn should_fail() {
  serial_print!("should_panic::should_fail...\t");
  assert_eq!(0, 1);
}