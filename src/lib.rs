#![feature(fn_traits, unboxed_closures, strict_provenance, downcast_unchecked)]

#[macro_use]
pub mod signal;
#[macro_use]
pub mod advanced_signal;
pub use signal::Signal;

pub mod prelude {
    pub use super::advanced_signal::*;
    pub use super::signal::*;
}
