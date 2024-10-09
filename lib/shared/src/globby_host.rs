extern crate std;

use std::sync::Mutex;

pub struct Globby<T> {
  thing: Mutex<Option<T>>,
}

impl <T> Globby<T> {
    pub const fn new() -> Globby<T> {
        Globby {
            thing: Mutex::new(None),
        }
    }

    pub fn set(&self, thing: T) {
        *(self.thing.lock().unwrap()) = Some(thing);
    }

    pub fn clear(&self) {
        *(self.thing.lock().unwrap()) = None;
    }

    pub fn use_thing<F>(&self, f: F)
    where
        F: FnOnce(&mut T) {
        if let Some(ref mut thing) = *(self.thing.lock().unwrap()) {
            f(thing);
        }
    }
}
