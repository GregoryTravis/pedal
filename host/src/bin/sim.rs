use host::sim::*;
use shared::filter::reso::*;

pub fn main() {
    sim_main(Box::new(ResoFilter::new()));
}
