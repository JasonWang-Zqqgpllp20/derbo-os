use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;      // set stack 0 as the default fault stack

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {        // define a TSS
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;     // set the top address because stacks on x86 grow downwards
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];   // define the 0th IST entry as the double fault stack

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });    // the compiler can't guarantee race freedom when mutable static are accessed
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {  // define a GDT
        // GDT is used for two things: switching between kernel space and user space; loading a TSS structure
        let mut gdt = GlobalDescriptorTable::new();

        // since we changed out GDT, we need to let the old selector point to a different GDT descriptor
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();    // load the GDT
    // reload CS and TSS registers
    unsafe {    // the two functions are unsafe, the reason maybe possible to break memory safety by loading invalid selectors
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}