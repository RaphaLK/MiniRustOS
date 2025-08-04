#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::panic::{PanicInfo};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use minirustos::{serial_println, exit_qemu, QemuExitCode};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> !{
  serial_println!("stack_overflow::stack_overflow...\t");

  minirustos::gdt::init();
  init_test_idt();

  stack_overflow();

  panic!("Execution continued after stack overflow");
}


lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler)
                .set_stack_index(minirustos::gdt::DOUBLE_FAULT_IST_INDEX); // Caller must ensure that the used index is valid and not already used for another exception
        }

        idt
    };
}

pub fn init_test_idt() {
  TEST_IDT.load()
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> !{
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop{}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow();
  volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  minirustos::test_panic_handler(info)
}