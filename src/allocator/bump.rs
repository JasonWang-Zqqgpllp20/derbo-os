use alloc::alloc::{GlobalAlloc, Layout};
use super::{align_up, Locked};
use core::ptr;

pub struct BumpAllocator {  // record a whole heap memory region
    heap_start: usize,      // the start and end keep track of the lower and upper bound of the region
    heap_end: usize,        // so only the addresses in the region is valid
    next: usize,            // record the address of the 'next' pointer when a new 'alloc' is called 
    allocations: usize,     // count the number of the alloc which hasn't been dealloced
}

impl BumpAllocator {
    pub const fn new() -> Self {    // create a new empty bump allocator
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {    // unsafe, for invalid memory return
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    // can't implement traits in differenct crates for GlobalAlloc
    // so we implement it indirectly by using Locked<> which is defined in allocator.rs (the same crate)
    unsafe fn alloc(&self, layout:Layout) -> *mut u8 {
        let mut bump = self.lock();     // get a mutable reference to the wrapped allocator type
        
        let alloc_start = align_up(bump.next, layout.align());  // align the start pointer
        let alloc_end = match alloc_start.checked_add(layout.size()) {// check integer overflow by num::CheckedAdd()
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {  // check if the end address of a single alloc is out of allocator boundary
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();     // get a mutable reference to the wrapped allocator type

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}