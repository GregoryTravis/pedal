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

    pub fn use_and_return<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R {
        //f(&mut self.thing.lock().expect("Could not lock mutex"))
        f(&mut self.thing.lock().unwrap())
    }
}
