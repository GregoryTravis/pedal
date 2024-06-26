extern crate alloc;

use alloc::boxed::Box;
use alloc::sync::Arc;

use shared::filter::reso::*;
use shared::rig::*;
use shared::signal::base::*;
use shared::signal::combinators::*;
use crate::daisy_seed::*;
use shared::constants::*;
use shared::glep;
use shared::load::*;
use shared::spew::*;

#[no_mangle]
pub fn main() {
    hw_init(true, BLOCK_SIZE);
    glep!("hi");
    load_init();

    let siner = PostCompose { signal: Arc::new(Sin {}), ff: scale_range(0.3, 0.9) };
    let q = Const { x: 0.95 };
    rig_install_patch(Box::new(ResoFilter::new(Arc::new(siner), Arc::new(q))));

    rig_install_callback();

    loop {
        patch_main();
    }
}
