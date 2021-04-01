#[cfg(feature = "event")]
extern crate dyn_clone;
#[cfg(feature = "event")]
extern crate once_cell;
#[cfg(feature = "event")]
extern crate parking_lot;
extern crate plctag_sys;

use plctag_sys as ffi;

pub mod builder;
mod debug;
#[cfg(feature = "event")]
pub mod event;
mod raw;
mod status;
#[cfg(feature = "value")]
mod value;

pub type Result<T> = std::result::Result<T, Status>;
pub use raw::RawTag;
pub use status::Status;

#[cfg(feature = "value")]
pub use value::{GetValue, SetValue};
