use alloc::boxed::Box;

use crate::spew::*;

pub trait Switches: Send {
    fn read(&self, switch_id: usize) -> bool;
    fn process(&self);
    fn spew(&self) {
        spew!("switches", self.read(0), self.read(1));
    }
}

// Toggle always starts in the false state.
pub struct Toggle {
    switches: Box<dyn Switches>,
    switch_id: usize,
    state: bool,
    last_pressed: bool,
}

impl Toggle {
    pub fn new(switches: Box<dyn Switches>, switch_id: usize) -> Toggle {
        Toggle {
            switches: switches,
            switch_id: switch_id,
            state: false,
            last_pressed: false,
        }
    }

    pub fn process(&mut self) {
        self.switches.process();
        let pressed = self.switches.read(self.switch_id);
        if pressed && !self.last_pressed {
            self.state = !self.state;
        }
        self.last_pressed  = pressed;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn spew(&self) {
        self.switches.spew();
        spew!("toggle", self.switch_id, self.state);
    }
}
