use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::constants::*;
use crate::knob::Knobs;
use crate::patch::Patch;
use crate::playhead::Playhead;

pub struct MixerChannel(pub f32, pub Box<dyn Patch>);

pub struct Mixer {
    channels: Vec<MixerChannel>,
    buffer: [f32; BLOCK_SIZE],
}

impl Mixer {
    // Scales the channel faders to sum to 1.0.
    pub fn new(mut channels: Vec<MixerChannel>) -> Mixer {
        let total_gain: f32 = channels.iter().map(|c| c.0).sum();
        assert!(total_gain != 0.0);
        //let adjusted_channels: Vec<MixerChannel> = Vec::with_capacity(channels.len());
        for i in 0..channels.len() {
            let ch: &mut MixerChannel = &mut channels[i];
            ch.0 /= total_gain;
            ch.0 *= 3.0;
        }

        Mixer {
            channels: channels,
            buffer: [0.0; BLOCK_SIZE],
        }
    }
}

impl Patch for Mixer {
    fn rust_process_audio(
        &mut self,
        input_slice: &[f32],
        output_slice: &mut [f32],
        knobs: &Box<dyn Knobs>,
        playhead: Playhead,
    ) {
        assert!(input_slice.len() == BLOCK_SIZE);
        assert!(output_slice.len() == BLOCK_SIZE);

        output_slice.iter_mut().for_each(|xp| *xp = 0.0);

        for channel in &mut self.channels {
            channel.1.rust_process_audio(input_slice, &mut self.buffer, knobs, playhead);
            for i in 0..output_slice.len() {
                output_slice[i] += channel.0 * self.buffer[i];
            }
        }
    }
}
