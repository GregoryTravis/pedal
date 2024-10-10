extern crate std;

use std::sync::Mutex;

pub struct Globby<T> {
  thing: Mutex<Option<T>>,
}

impl <T> Globby<T> {
    pub const fn empty() -> Globby<T> {
        Globby {
            thing: Mutex::new(None),
        }
    }

    pub const fn new(t: T) -> Globby<T> {
        Globby {
            thing: Mutex::new(Some(t)),
        }
    }

    pub fn set(&self, thing: T) {
        //interrupt::free(|cs| self.thing.borrow(cs).replace(Some(thing)));
        self.lala(|mor| {
            *mor = Some(thing);
        });
    }

    pub fn clear(&self) {
        //interrupt::free(|cs| { self.thing.borrow(cs).replace(None); });
        self.lala(|mor| {
            *mor = None;
        });
    }

    pub fn use_thing<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R {
        self.lala(|mor| {
            if let Some(ref mut thing) = mor.as_mut() {
                f(thing)
            } else {
                todo!();
            }
            //f(mor.as_mut())
        })
        /*
        interrupt::free(|cs| {
            if let Some(ref mut thing) = self.thing.borrow(cs).borrow_mut().deref_mut().as_mut() {
                f(thing)
            } else {
                todo!()
            }
        })
        */
    }

    fn lala<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R {
        //f(&mut self.thing.lock().expect("Could not lock mutex"))
        f(&mut self.thing.lock().unwrap())
    }
}
