extern crate alloc;

use alloc::boxed::Box;
use core::cmp::min;

use crate::constants::*;
use crate::patch::Patch;
use crate::rig::*;

pub fn rig_run_patch_on_buffer(patch: Box<dyn Patch>, input: &[f32], output: &mut [f32]) {
    let len = input.len();

    rig_install_patch(patch);

    let mut sofar = 0;
    while sofar < len {
        let start = sofar;
        let end = min(sofar + BLOCK_SIZE, len);
        let block_length = end-start;
        //println!("rpob {} {} {} {}", sofar, len, start, end);
        let sub_input = &input[start..end];
        let mut sub_output: &mut [f32] = &mut output[start..end];
//let body_slice: &mut [u8] = &mut myvec[10..1034];

        //println!("rpob {} {}", sub_input.len(), sub_output.len());
        rust_process_audio_soft(&sub_input, &mut sub_output, block_length);
        //println!("rpob 2 {} {}", sub_input.len(), sub_output.len());

        sofar += BLOCK_SIZE;
        //println!("rpob hey");
    }

    rig_deinstall_patch();
}
