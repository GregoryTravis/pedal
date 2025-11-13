#[cfg(feature = "for_host")]
extern crate std;
extern crate libm;

pub struct Inertial {
    current: f32,
    target: f32,
    max_delta_per_update: f32,
}

impl Inertial {
    pub fn new(x: f32, max_delta_per_update: f32) -> Inertial {
        Inertial::new_from(x, x, max_delta_per_update)
    }
    pub fn new_from(current: f32, target: f32, max_delta_per_update: f32) -> Inertial {
        Inertial {
            current: current,
            target: target,
            max_delta_per_update: max_delta_per_update,
        }
    }

    pub fn get(&self) -> f32 {
        self.current
    }

    pub fn set(&mut self, x: f32) {
        self.target = x;
    }

    pub fn update(&mut self) {
        if self.target > self.current {
            let dist = self.target - self.current;
            let delta: f32 = if dist < self.max_delta_per_update { dist } else { self.max_delta_per_update };
            self.current += delta;
        } else {
            let dist = self.current - self.target;
            let delta: f32 = if dist < self.max_delta_per_update { dist } else { self.max_delta_per_update };
            self.current -= delta;
        }
    }
}
