use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{self, Mutex};

pub struct Globby<T> {
  thing: Mutex<RefCell<Option<T>>>,
}

impl <T> Globby<T> {
    pub const fn empty() -> Globby<T> {
        Globby {
            thing: Mutex::new(RefCell::new(None)),
        }
    }

    pub const fn new(t: T) -> Globby<T> {
        Globby {
            thing: Mutex::new(RefCell::new(Some(t))),
        }
    }

    pub fn use_and_return<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R {
        interrupt::free(|cs| {
            let mut binding = self.thing.borrow(cs).borrow_mut();
            let thing: &mut Option<T> = binding.deref_mut();
            f(thing)
        })
    }
}
