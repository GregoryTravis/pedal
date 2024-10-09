extern crate std;

use std::thread;

use crate::constants::BLOCK_SIZE;
use crate::globby::*;
use crate::rig::*;
use crate::rig_type::Rig;

pub static THE_PATCH: Globby<Rig> = Globby::new();

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
