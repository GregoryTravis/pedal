#![cfg_attr(not(for_host), no_std)]

#[macro_use]
extern crate alloc;

pub mod ds;
pub mod convert;
pub mod constants;
pub mod filter;
#[cfg(feature = "for_host")]
pub mod graphing;
pub mod load;
pub mod panic;
pub mod patch;
pub mod playhead;
pub mod rig;
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
