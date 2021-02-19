use {
    std::{
        alloc::{
            GlobalAlloc,
            Layout,
            System,
        },
    },
};

/// just an allocator that doesn't desallocate.
///
/// With the current behavior of rhit it makes sense
/// and makes the total execution 10% to 15% faster.
///
/// This may disappear in a future release when rhit
/// has more needs to reuse memory.
pub struct LeakingAllocator {}

impl LeakingAllocator {
    pub const fn new() -> Self {
        Self {}
    }
}

unsafe impl GlobalAlloc for LeakingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}
