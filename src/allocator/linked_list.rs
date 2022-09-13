use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,    // &'static describes an owned object behind a pointer 
}

impl ListNode {
    const fn new(size: usize) -> Self { // define as a const fn for cunstructing a static linked list allocator
        ListNode { size, next:None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize    // the start address of the ListNode
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    // the real allocator initializing funtion is new() rather than init(), because new() is used for ALLOCATOR
    pub const fn new() -> Self {    // create an empty LiknedListAllocator
        Self {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {    // initialize the allocator
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {// add the given region to the front of the list
        assert_eq!( align_up(addr, mem::align_of::<ListNode>()), addr ); // the func return the alignment of the ListNode structure
        assert!( size >= mem::size_of::<ListNode>() );      // return a size of the ListNode structure

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();  // Option::take() takes the value out of the option, leaves a None in its place
        
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node); // ptr::write() overwrites a memory location with the given value without reading
        self.head.next = Some(&mut *node_ptr);
    }

    fn find_region(&mut self, size: usize, align: usize) // try to find a suitable empty region with the given info
        -> Option<(&'static mut ListNode, usize)> 
    {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next { // look for a suitable memory region
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {    // find ok
                let next = region.next.take();  // take region.next to a variable and set it as None
                let ret = Some((current.next.take().unwrap(), alloc_start)); // take current.next
                current.next = next;            // relink nodes after taking the suitable node out
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None // no suitable region found
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) // check the region is ok or not
        -> Result<usize, ()> 
    {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;   // check the overflow

        if alloc_end > region.end_addr() {  // the region is too small
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;          // check the remain capability
        // the LinkedListAllocator needs to store the ListNode itself
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // rest of region too small to hold a ListNode (required because the allocation splits the region in a used and a free part)
            return Err(());
        }

        Ok(alloc_start)
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())  // increase the alignment to the alignment of ListNode if necessary
            .expect("adjusting alignment failed")
            .pad_to_align(); // round up the size to a multiple of the alignment to ensure that ensure that 
            //the start address of the next memory block will have the correct alignment for storing a ListNode too
        let size = layout.size().max(mem::size_of::<ListNode>());

        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) { // find a suitable region and remove it from the list
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {    // divede the region into an used one and a free one
                allocator.add_free_region(alloc_end, excess_size);
            }

            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);

        self.lock().add_free_region(ptr as usize, size) // add the dealloc region to the free list
    }
}