#![cfg_attr(not(for_host), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(not(feature = "for_host"))]
pub mod mem;
pub mod ds;
pub mod convert;
pub mod constants;
pub mod filter;
#[cfg(feature = "for_host")]
pub mod graphing;
#[cfg(feature = "for_host")]
pub mod hw_host;
pub mod load;
pub mod r#override;
pub mod panic;
pub mod patch;
pub mod playhead;
pub mod rig;
pub mod rig_util;
#[cfg(not(feature = "for_host"))]
pub mod rig_board;
#[cfg(feature = "for_host")]
pub mod rig_host;
pub mod rig_type;
pub mod signal;
#[cfg(feature = "for_host")]
pub mod sim;
pub mod speed_test;
pub mod spew;
#[cfg(not(feature = "for_host"))]
pub mod spew_board;
#[cfg(feature = "for_host")]
pub mod spew_host;
pub mod test;
pub mod testdata;
#[cfg(feature = "for_host")]
pub mod testdump;
pub mod testutil;
