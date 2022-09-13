#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
// #![feature(asm)] // could enable the experimental asm! macro for inline assembly
// #![feature(asm_const)]
// #![feature(llvm_asm)]
#![feature(custom_test_frameworks)]
#![test_runner(DerBo_OS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use DerBo_OS::{print, println};
use DerBo_OS::task::{Task, executor::Executor};

entry_point!(kernel_main);

#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! { // completely Rust funtion VS extern "C" _start
    println!("Terminal: 1/2");
    print!("root>");

    use DerBo_OS::buffer::vga_buffer::INITIAL;
    unsafe { INITIAL = true; }
    
    DerBo_OS::init();     // call the function in lib.rs to use interrupt::init_idt() indirectly

    /* init the heap */
    use x86_64::VirtAddr;
    use DerBo_OS::memory;
    use DerBo_OS::memory::BootInfoFrameAllocator;
    use DerBo_OS::allocator;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);  // get the offset
    let mut mapper = unsafe { memory::init(phys_mem_offset) };  // get a offset page table
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    /* trigger a breakpoint exception */
    // x86_64::instructions::interrupts::int3(); 

    /* trigger a double fault */
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42; 
    // };

    /* trigger a stack overflow */
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();   

    /* test the data race for print macro */
    // loop {
    //     use DerBo_OS::print;
    //     // for _ in 0..300000 {}   // set a short interval for vga_buffer::_print()
    //     print!("-");
    // }

    /* trigger a page fault */
    // let ptr = 0xdeadbeaf as *mut u32;   
    // unsafe { *ptr = 42; }

    /* test the read and write of the physical address */
    // let ptr = 0x204fb1 as *mut u32;   // 0x204fb1 is the page that be pointed when the page fault occurs
    // unsafe { let x = *ptr; }    // read from a code page
    // println!("read worked");
    // unsafe { *ptr = 42; }       // write to a code page
    // println!("write worked");

    /* get the start address of l4_page_table through Cr3 register */
    // use x86_64::registers::control::Cr3;         
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    /* get the entries of l4 table and l3 table and print in the QEMU */
    // use DerBo_OS::memory::active_level_4_table;
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);  // get the offset
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) }; // get entries array of the level 4 page table
    // for (i, entry) in l4_table.iter().enumerate() { // iterate table entries array
    //     use x86_64::structures::paging::PageTable;

    //     if !entry.is_unused() {                     // only output the used entries
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    /* test for computing the physical address of a virtual address */
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to pyhsical address 0
    //     boot_info.physical_memory_offset,
    // ];
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    
    /* test for creating a new mapping for the VGA buffer at address 0 */
    // // create an unused page
    // use x86_64::structures::paging::Page;
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));      // create a page at virtual address xx
    // // creat a map, from virt addr 0 to phys addr 0xb8000 which is specified in funciton create_example_mapping
    // memory::create_example_mapping(0xb8000, page, &mut mapper, &mut frame_allocator);    
    // // write the string 'New!' to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr(); // convert the page to a raw pointer
    // unsafe { page_ptr.offset(20).write_volatile(0x_f021_f077_f065_f04e) }; // write to VGA with a adding offset
    // // if we don't add an offset, the address (0xb8000 + 0) will be the first line which can't be seen in the QEMU
    // // "0x_f021_f077_f065_f04e" represents "New!"
    


    /*
    // allocate a number on the heap
    let heap_value = Box::new(41);   // panic as espect because the alloc() always returns a null pointer
    println!("heap_value at {:p}", heap_value);

    // craete a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference couted vector which will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));*/

    /* test for the net port */
    // use x86_64::instructions::port::Port;
    // let mut port = Port::new(0x366);
    // for i in 0..10 {
    //     let scancode: u8 = unsafe { port.read() };
    //     println!("net:{}", scancode);
    // }

    /* test for the async funtion and async keyboard input */
    // async fn async_number() -> u32 {
    //     42
    // }
    // async fn example_task() {
    //     let number = async_number().await;
    //     println!("async number: {}", number);
    // }

    let mut executor = Executor::new();
    executor.spawn(Task::new(DerBo_OS::buffer::keyboard::print_keypresses()));
    executor.spawn(Task::new(DerBo_OS::terminal::terminal1::print_keypresses()));
    executor.spawn(Task::new(DerBo_OS::terminal::task1::run_command()));
    executor.spawn(Task::new(DerBo_OS::terminal::terminal2::print_keypresses()));
    executor.spawn(Task::new(DerBo_OS::terminal::task2::run_command()));
    executor.spawn(Task::new(DerBo_OS::timer::cursor::print_timerfifo()));
    executor.run();     // loop

    #[cfg(test)]
    test_main();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    DerBo_OS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    DerBo_OS::test_panic_handler(info)
}