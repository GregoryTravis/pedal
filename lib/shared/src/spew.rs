#[cfg(feature = "for_host")]
pub use crate::spew_host::*;

#[cfg(not(feature = "for_host"))]
pub use crate::spew_board::*;

// TODO: don't print a space after the last argument.
#[macro_export]
macro_rules! spew {
    ($($args:expr),*) => {{
        $($args.do_spew();
          spew_space();
          )*
        spew_newline();
    }};
}

pub use spew;

pub struct Hex(pub u64);
pub fn hex(x: u64) -> Hex { Hex(x) }
