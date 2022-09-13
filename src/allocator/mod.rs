use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use fixed_size_block::FixedSizeBlockAllocator;

pub mod bump;
pub mod linked_list;
pub mod fixed_size_block;

pub struct Dummy;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1000 * 1024;    // 1000 KiB

#[global_allocator] // tell the Rust compiler which allocator instance it should use as the global heap allocator
// static ALLOCATOR: LockedHeap = LockedHeap::empty();
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new()); // use the Spinlock type to deal with multiple threads
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()  // always return a null pointer
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)   // create a page range from the start and end pages
    };

    for page in page_range {        // map all pages of the page range above
        let frame = frame_allocator
            .allocate_frame()       // FrameAllocator::allocate_frame(): allocate a frame of the appropriate size
            .ok_or(MapToError::FrameAllocationFailed)?;     // return an error if possible
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE; // both read and write accesses are allowed
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush() // flush the page from the TLB to newest
        };
    }

    unsafe {    // initialize the heap after mapping the heap pages since the init() tries to write to the heap
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

pub struct Locked<A> {      // a wrapper around a spin::Mutex<A>
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }
    
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {   // align the given 'addr' unwards to alignment 'align'
    // let remainder = addr % align;
    // if remainder == 0 {
    //     addr
    // } else {
    //     addr - remainder + align
    // }
    (addr + align - 1) & !(align - 1)   // more efficient but hard to understand
}