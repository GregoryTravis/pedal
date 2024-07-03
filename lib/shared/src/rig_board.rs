use crate::rig_type::Rig;

//use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{self, Mutex};

static THE_PATCH: Mutex<RefCell<Option<Rig>>> = Mutex::new(RefCell::new(None));

pub fn rig_set(rig: Rig) {
    interrupt::free(|cs| THE_PATCH.borrow(cs).replace(Some(rig)));
}

pub fn rig_clear() {
    interrupt::free(|cs| {
        if let Some(ref mut _rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
            THE_PATCH.borrow(cs).replace(None);
        }
    });
}

pub fn rig_use<F>(f: F)
where
    F: FnOnce(&mut Rig) {
    interrupt::free(|cs| {
        if let Some(ref mut rig) = THE_PATCH.borrow(cs).borrow_mut().deref_mut().as_mut() {
            f(rig);
        }
    });
}
