#[cfg(feature = "for_host")]
pub use crate::spew_host::*;

#[cfg(not(feature = "for_host"))]
pub use crate::spew_board::*;

#[macro_export]
macro_rules! spew {
    ($($args:expr),*) => {{
        $($args.do_spew();
          spew_space();
          )*
        spew_newline();
    }};
}
