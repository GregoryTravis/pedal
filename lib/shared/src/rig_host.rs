extern crate std;
extern crate lazy_static;

use std::sync::Mutex;
use std::thread;
use lazy_static::lazy_static;

use crate::constants::BLOCK_SIZE;
use crate::rig::*;
use crate::rig_type::Rig;

lazy_static! {
    //static ref THE_PATCH: Mutex<RefCell<Option<Rig>>> = Mutex::new(RefCell::new(None));
    static ref THE_PATCH: Mutex<Option<Rig>> = Mutex::new(None);
}
//static THE_PATCH: Mutex<RefCell<Option<Rig>>> = Mutex::new(RefCell::new(None));

pub fn rig_set(rig: Rig) {
    *(THE_PATCH.lock().unwrap()) = Some(rig);
    //let x: RefCell<Option<Rig>> = (THE_PATCH.lock().unwrap());
    //*(x.borrow_mut()) = Some(rig);

    //let mut ref_cell: RefCell<Option<Rig>> = THE_PATCH.lock().unwrap();
    //ref_cell.replace(Some(rig));
}

pub fn rig_clear() {
    *(THE_PATCH.lock().unwrap()) = None;
    /*
    let mut ref_cell = THE_PATCH.lock();
    ref_cell.replace(None);
    */
}

pub fn rig_use<F>(f: F)
where
    F: FnOnce(&mut Rig) {
    if let Some(ref mut rig) = *(THE_PATCH.lock().unwrap()) {
        f(rig);
    }

        /*
    let mut ref_cell = THE_PATCH.lock();
    if let Some(ref mut rig) = THE_PATCH.lock.borrow_mut().deref_mut().as_mut() {
        f(rig);
    }
    */
}

pub fn rig_install_callback() {
    let _handler = thread::spawn(|| {
        let input: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];
        let mut output: [f32; BLOCK_SIZE] = [0.0; BLOCK_SIZE];
        loop {
            rust_process_audio_soft(&input, &mut output, BLOCK_SIZE);
        }
    });

    // TODO stop the thread when done
    // handler.join().unwrap();
}
