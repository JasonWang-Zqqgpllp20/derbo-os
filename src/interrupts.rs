use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::structures::idt::PageFaultErrorCode;
use pic8259::ChainedPics;
use crate::println;
use crate::gdt;
use crate::hlt_loop;
use spin;
use lazy_static::lazy_static;

pub const PIC_1_OFFSET: u8 = 32;                // the first byte before 0~31, as primary interrupt controller
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;  // the second byte before 0~31, as secondary interrupt controller

pub static PICS: spin::Mutex<ChainedPics> =     // define a static PIC with two interrupt controllers
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub static mut TIMER_COUNT: u64 = 0; // 2^64 * 0.05 is enough for stimulation

#[derive(Debug, Clone, Copy)]
#[repr(u8)]     // each variant is represented as an u8
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,   // the timer field
    Keyboard,               // the keyboard field, it defaults to plus one---interrupt 33(1 + offset32)
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    // define the IDT static because the cpu access IDT on every interrupt after loading it
    // use a lazy_static! to initial IDT when the static is referenced the first time rather than use 'unsafe + mut'
    static ref IDT: InterruptDescriptorTable = {        // we should remapping the interrupt numbers
        let mut idt = InterruptDescriptorTable::new();
        
        idt.breakpoint.set_handler_fn(breakpoint_handler);      // set breakpoint handler
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)   // set double fault handler
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);      // set the default stack
        }
        idt[InterruptIndex::Timer.as_usize()]           // set timer interrupt handler
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]        // set keyboard interrupt handler
            .set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);  // set page fault handler
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(   // define a breakpoint handler
    _stack_frame: InterruptStackFrame)           // the book use '&mut', but there is an error
{
    // println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    // println!("RFLAGS: {:?}", x86_64::registers::rflags::read());
    println!("RFLAGS statu: {:?}", x86_64::registers::rflags::read_raw());
    println!("CS index: {:?}", x86_64::instructions::segmentation::cs().0);
}

extern "x86-interrupt" fn double_fault_handler( // define a double fault handler
    stack_frame: InterruptStackFrame, _error_code: u64) -> ! // the book use '&mut', but there is an error
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(  // define a timer interrupt handler which is already enabled by default
    _stack_frame: InterruptStackFrame) // the book use '&mut', but there is an error
{
    // print!(".");
    use crate::timer::cursor;
    use crate::timer::sleep;
    unsafe {
        TIMER_COUNT += 1;         // the timer cycle is set to about 0.05s, so 20T ≈ 1s.
        if TIMER_COUNT % 10 == 0 {                   // for cursor timer
            cursor::timerfifo_push(0x01);
        }
        
        sleep::timerfifo_push(TIMER_COUNT);   // for sleep timer
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); // send the EIO to tell the controller that the system is ready to receive the next interrupt
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::Port;
    use crate::terminal::terminal1::TASKING_1;
    use crate::terminal::terminal2::TASKING_2;
    use crate::terminal::SwitchState;
    use crate::buffer::keyboard::SWITCH;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::buffer::keyboard::add_scancode(scancode);
    unsafe {
        if TASKING_1 == false && SWITCH == SwitchState::Terminal1 {
            crate::terminal::terminal1::add_scancode(scancode);
        }
        if TASKING_2 == false && SWITCH == SwitchState::Terminal2 {
            crate::terminal::terminal2::add_scancode(scancode);
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());     // send the EIO
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

#[test_case]
fn test_breakpoint_exception() {// adding a test for interrupt::init_idt(), use the command 'cargo test --lib'
    x86_64::instructions::interrupts::int3();
}