extern crate alloc;

// TODO: make spewing never allocate, then make this a logging allocator wrapper, under
// a flag.
use core::alloc::{GlobalAlloc, Layout};
use core::cell::{Ref,RefCell,RefMut};
use cortex_m::interrupt::{self, Mutex};

use crate::spew::*;

const HEAP_SIZE: usize = 32768;
const LOGGING: bool = false;

// We assume that the underlying allocator has 0 overhead, which means that we will
// underestimate how much is allocated and sometimes fail to reject an allocation that
// cannot succeed.
//
// The underlying `emballoc` allocator fails gracefully for small overallocations, so that
// will handle the case of us underestimating.
#[derive(Copy, Clone, Debug)]
struct Stats {
    currently_allocated: usize,
    total_allocated: usize,
    total_deallocated: usize,
    num_allocations: usize,
    num_deallocations: usize,
}

impl Stats {
    pub const fn new() -> Stats {
        Stats {
            // Currently allocated, not including allocator overhead
            currently_allocated: 0,
            // Sum of all allocations
            total_allocated: 0,
            // Sum of all deallocations
            total_deallocated: 0,
            num_allocations: 0,
            num_deallocations: 0,
        }
    }
}

struct MyAlloc {
    stats: Mutex<RefCell<Stats>>,
    allocator: emballoc::Allocator<HEAP_SIZE>,
}

impl MyAlloc {
    const fn new() -> MyAlloc {
        MyAlloc {
            stats: Mutex::new(RefCell::new(Stats::new())),
            allocator: emballoc::Allocator::new()
        }
    }
}

impl MyAlloc {
    fn report(&self) {
        let mut stats_copy: Stats = Stats::new();
        interrupt::free(|cs| {
            let stats: Ref<Stats> = self.stats.borrow(cs).borrow(); // .borrow_mut().deref_mut().as_mut();
            stats_copy = *stats;
        });
        spew!("mem",
              (stats_copy).currently_allocated,
              (stats_copy).total_allocated,
              (stats_copy).total_deallocated,
              (stats_copy).num_allocations,
              (stats_copy).num_deallocations
              );
    }
}

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        interrupt::free(|cs| {
            let mut stats: RefMut<Stats> = self.stats.borrow(cs).borrow_mut(); // .borrow_mut().deref_mut().as_mut();

            (*stats).currently_allocated += size;
            (*stats).total_allocated += size;
            (*stats).num_allocations += 1;

            if (*stats).currently_allocated > HEAP_SIZE {
                panic!("Out of memory (0)");
            }
        });

        if LOGGING {
            spew!("mem alloc", size);
            self.report();
        }

        let r = self.allocator.alloc(layout);
        if r.is_null() {
            spew!("Out of memory!");
            if LOGGING {
                self.report();
            }
            panic!("Out of memory (1)");
        }
        r
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();

        interrupt::free(|cs| {
            let mut stats: RefMut<Stats> = self.stats.borrow(cs).borrow_mut(); // .borrow_mut().deref_mut().as_mut();
            (*stats).currently_allocated -= size;
            (*stats).total_deallocated += size;
            (*stats).num_deallocations += 1;
        });

        self.allocator.dealloc(ptr, layout);
    }
}

#[global_allocator]
static ALLOCATOR: MyAlloc = MyAlloc::new();
