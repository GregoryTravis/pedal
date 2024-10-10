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

    pub fn set(&self, thing: T) {
        interrupt::free(|cs| self.thing.borrow(cs).replace(Some(thing)));
    }

    pub fn clear(&self) {
        interrupt::free(|cs| {
            self.thing.borrow(cs).replace(None);
        });
    }

    pub fn use_thing<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R {
        interrupt::free(|cs| {
            if let Some(ref mut thing) = self.thing.borrow(cs).borrow_mut().deref_mut().as_mut() {
                f(thing)
            } else {
                todo!()
            }
        })
    }

    fn lala<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R {
        interrupt::free(|cs| {
            let mut binding = self.thing.borrow(cs).borrow_mut();
            let thing: &mut Option<T> = binding.deref_mut();
            //let thing: &mut Option<T> = self.thing.borrow(cs).borrow_mut().deref_mut();
            f(thing)
        })
    }
}
