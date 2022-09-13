#![no_std]
#![feature(core_intrinsics)]
#![feature(box_into_inner)]
// #![feature(asm)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]         // set test_runner in lib.rs as 'test_main'
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]    // specify a funtion that is called when an allocation error occurs
#![feature(const_mut_refs)]         // for 'const fn new()' in allocator/linked_list.rs

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

pub mod serial;
pub mod buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod timer;
pub mod terminal;
pub mod api;
pub mod file;
pub mod compiler;

#[cfg(test)]
entry_point!(test_kernel_main);

// insert automatical printing-info
pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) { // print two universal info and a self-defined info
        serial_print!("{}...\t", core::any::type_name::<T>()); //  print the function name by any::type_name
        self();                     // self need to implements the Fn() trait and call the print function itself
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) { 
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();     // run() is the function of Testable Trait
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}




#[cfg(test)]
#[no_mangle] // don't mangle the name of this function
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {           // named `_start` by default
    init();     // call the interrupt::init_idt() for the test in 'cargo test --lib'

    test_main();
    
    hlt_loop();   
}

pub fn init() {     // use interrupt::init_idt() indirectly
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); // execute the 'sti' instruction to enable external interrupts
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// panic for 'cargo test'
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// for exiting the QEMU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}