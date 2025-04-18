use hashbrown::HashMap;

use crate::spew::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Peaks,
    Bps,
    OldFreqs,
    OldFaves,
    NewFaves,
    Results,
}

pub struct Maxes {
    counts: HashMap<Item, usize>,
}

impl Maxes {
    pub fn new() -> Maxes {
        Maxes {
            counts: HashMap::new(),
        }
    }

    pub fn update(&mut self, item: Item, value: usize) {
        let current = *self.counts.get(&item).unwrap_or(&0);
        self.counts.insert(item, value.max(current));
        //spew!("maxes update", &format!("{:?}", item).as_str(), self.counts.get(&item).unwrap());
    }

    pub fn dump(&self) {
        for (k, v) in self.counts.iter() {
            spew!("maxes", format!("{:?}", k).as_str(), v);
        }
    }
}
