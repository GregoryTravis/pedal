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
            let delta: f32 = libm::fminf(self.target - self.current, self.max_delta_per_update);
            self.current += delta;
        } else {
            let delta: f32 = libm::fminf(self.current - self.target, self.max_delta_per_update);
            self.current -= delta;
        }
    }
}
