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
        //let _i0: i32 = *(self.thing.lock().unwrap()); // option<t>
        //let _i01: &mut Option<T> = (self.thing.lock().unwrap()); //
        //let _i1: i32 = self.thing.lock().unwrap(); // MutexGuard<'_, Option<T>>
        ////let _i11: i32 = self.thing.lock().unwrap().deref(); // 
        //let _i2: i32 = self.thing.lock(); // Result<MutexGuard<'_, ...>, ...>
        if let Some(ref mut thing) = *(self.thing.lock().unwrap()) {
            f(thing)
        } else {
            todo!()
        }
    }

    fn lala<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<T>) -> R {
        //f(&mut self.thing.lock().expect("Could not lock mutex"))
        f(&mut self.thing.lock().unwrap())
    }
}
