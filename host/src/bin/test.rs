extern crate alloc;

use shared::r#override::*;
#[allow(unused_imports)]
use shared::test::*;

pub fn main() {
    test_direct();
    run_override_test();
}
