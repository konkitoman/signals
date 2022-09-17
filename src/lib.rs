#![feature(fn_traits, unboxed_closures, strict_provenance, downcast_unchecked)]
#![feature(core_intrinsics)]

#[macro_use]
pub mod signal;
#[macro_use]
pub mod advanced_signal;
pub use signal::Signal;
#[macro_use]
pub mod get;

pub mod prelude {
    pub use super::advanced_signal::*;
    pub use super::get::TGetPtr;
    pub use super::signal::*;
    pub use super::*;
    pub use std::any::Any;
    pub use std::sync::{Arc, Mutex};
}
