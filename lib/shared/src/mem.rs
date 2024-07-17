extern crate alloc;

//use alloc::string::ToString;
use core::alloc::{GlobalAlloc, Layout};

use crate::spew::*;

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<32768> = emballoc::Allocator::new();

// TODO: make spewing never allocate, then make this a logging allocator wrapper, under
// a flag.
/*
struct MyAlloc {
    allocator: emballoc::Allocator<32768>,
}

impl MyAlloc {
    const fn new() -> MyAlloc {
        MyAlloc { allocator: emballoc::Allocator::new() }
    }
}

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: say "alloc" instead of "0", when spewing doesn't allocate.
        spew!(0, layout.size());
        let r = self.allocator.alloc(layout);
        if r.is_null() {
            spew!(0, 1, 2, 3, layout.size());
            // TODO: enable this when panic's spew doesn't allocate
            // panic!("out of mem");
        }
        r
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // TODO: say "dealloc" instead of "1", when spewing doesn't allocate.
        spew!(1, layout.size());
        self.allocator.dealloc(ptr, layout);
    }
}

#[global_allocator]
static ALLOCATOR: MyAlloc = MyAlloc::new();
*/
