#[cfg(not(feature = "for_host"))]
pub use crate::globby_board::*;
#[cfg(feature = "for_host")]
pub use crate::globby_host::*;

impl <T> Globby<T> {
    pub fn set(&self, thing: T) {
        //interrupt::free(|cs| self.thing.borrow(cs).replace(Some(thing)));
        self.use_and_return(|mor| {
            *mor = Some(thing);
        });
    }

    pub fn clear(&self) {
        //interrupt::free(|cs| { self.thing.borrow(cs).replace(None); });
        self.use_and_return(|mor| {
            *mor = None;
        });
    }

    pub fn use_it<F>(&self, f: F)
    where
        F: FnOnce(&mut T) {
            self.use_and_return(|mot| {
                if let Some(ref mut thing) = mot {
                    f(thing)
                }
            });
    }

    pub fn map<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R {
            self.use_and_return(|mot| {
                if let Some(ref mut thing) = mot {
                    Some(f(thing))
                } else {
                    None
                }
            })
    }
}
