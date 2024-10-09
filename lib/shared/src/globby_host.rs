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
        *(self.thing.lock().unwrap()) = Some(thing);
    }

    pub fn clear(&self) {
        *(self.thing.lock().unwrap()) = None;
    }

    pub fn use_thing<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R {
        if let Some(ref mut thing) = *(self.thing.lock().unwrap()) {
            f(thing)
        } else {
            todo!()
        }
    }
}
