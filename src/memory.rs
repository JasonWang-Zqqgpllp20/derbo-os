use x86_64::structures::paging::{PageTable, OffsetPageTable};
use x86_64::structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator};
use x86_64::{PhysAddr, VirtAddr};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

/*
This function is unsafe because the caller must guarantee that the complete physical memory 
is mapped to virtual memory at the passed `physical_memory_offset`. 
Also, this function must be only called once to avoid aliasing `&mut` references (which is undefined behavior).
*/
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read(); // read the physical frame of the active level 4 table from CR3

    let phys = level_4_table_frame.start_address(); // take the physical start address
    let virt = physical_memory_offset + phys.as_u64();  // convert to an u64, add the offset, to get virtual addr
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr(); // convert to a *mut PageTable raw pointer

    &mut *page_table_ptr    // unsafe, return a mutable reference to the level 4 page table
}

/*
This function is unsafe because the caller must guarantee that the complete physical memory 
is mapped to virtual memory at the passed `physical_memory_offset`. 
Also, this function must be only called once to avoid aliasing `&mut` references (which is undefined behavior).
*/
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {  // return a static instance
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

// pub struct EmptyFrameAllocator; // A FrameAllocator that always returns 'None', just to test our mapping function

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}
impl BootInfoFrameAllocator {   
    // create a FrameAllocator from the passed memory map
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,     // the memory map passed by the bootloader
            next: 0,        // keep the track of number of the next frame that the allocator should return
                            // the 'next' field is 0, and will be increased for every frame allocation
        }   // return a BootInfoFrameAllocator instance
    }

    // return an iterator over the usable frames specified in the memory map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();   // convert the memory map to an iterator of MemoryRegion
        let usable_regions = regions            // filter Usable regions rather than reserved or unavailable ones
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        
        let addr_ranges = usable_regions        // transform iterator of memory regions to that of address ranges
            .map(|r| r.range.start_addr()..r.range.end_addr());   // it will become 2 dementions
            
        // transform to an iterator of frame start addresses and then convert to a 1 demention array
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // convert the start addresses to PhysFrame types to construct the Iterator<Item = PhysFrame>
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator { // for the parameter in create_example_mapping()
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);// get all usable frames and get the frame with index 'next'
        self.next += 1;     // for the following frame on the next call
        frame               // return the allocated frame
    }
}

// a FrameAllocator that returns unsable frames from the bootloader's memory map


pub fn create_example_mapping(
    sourse_addr: u64,
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>, // generic over all types that implement the trait
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(sourse_addr));  // destination: 0xb8000, the start of VGA
    let flags = Flags::PRESENT | Flags::WRITABLE;       // set flags

    let map_to_result = unsafe {
        // the mapping only worked because the level 1 table responsible for the page at address 0 already exists
        mapper.map_to(page, frame, flags, frame_allocator)  // the func needs 4 parameters, then returns a Result<>
    };  // the 'map_to' is unsafe because the caller must ensure that the frame is not already in use

    map_to_result.expect("map_to failed").flush();
}












/* we no longer need our memory::translate_addr and memory::translate_addr_inner functions, we can delete them */
/*
This function is unsafe because the caller must guarantee that the complete physical memory 
is mapped to virtual memory at the passed `physical_memory_offset`. 
*/
// pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     translate_addr_inner(addr, physical_memory_offset)  // translate the virtual addr to the mapped physical addr
// }
// define a safe function and then wrap it in an unsafe block to limit the scope of unsafe (I can't understand)
// make the function private otherwise it can easily lead to aliased mutable references when called multiple times
// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     use x86_64::structures::paging::page_table::FrameError;
//     use x86_64::registers::control::Cr3;

//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indexes = [   // compute four level page tables of the given addr by shifting, e.g. xx >> 12 >> 9
//         addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
//     ];  // each function returns a page table with 9-bit index (2^9 = 512)
//     /*
//         for addr 0xb8000, the table_indexes will look like:
//         [ PageTableIndex(0,), PageTableIndex(0,), PageTableIndex(0,), PageTableIndex(184,) ]
//     */
//     let mut frame = level_4_table_frame;    // the start physical address of the for loop 

//     let mut i: i64 = 0;
//     // traverse the multi-level page table
//     for &index in &table_indexes {      // interate the l4, l3, l2, l1 page table of the given address
//         // convert the frame into a page table reference
//         let virt = physical_memory_offset + frame.start_address().as_u64(); // the truncated address also need to add a offset to become a physical address
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };     // update the table in every loop ( l4 -> l3 -> l2 -> l1 )

//         // read the page table entry and update frame
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,         // if the entry is in l1 table, it's a frame
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     Some(frame.start_address() + u64::from(addr.page_offset())) // add the offset to return a physical address
// }