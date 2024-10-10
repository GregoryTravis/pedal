#[cfg(not(feature = "for_host"))]
pub use crate::globby_board::*;
#[cfg(feature = "for_host")]
pub use crate::globby_host::*;

impl <T> Globby<T> {
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
}
