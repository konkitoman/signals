#![feature(fn_traits, unboxed_closures, strict_provenance)]
#![feature(core_intrinsics)]
//! # Signal
//! Is a simple signal to connect callbacks
//!
//! # AdvancedSignal
//! When you connect a callback you can set a other value
//! ## AdvanceSignal method is created with macro advanced_method.
//! ```rust
//!advanced_method!{
//!    fn method(|){
//!    
//!    }    
//!}
//!```
//! Function with state
//!```rust
//!use signals_kman::prelude::*;
//!
//!pub struct State {
//!    a: i32,
//!}
//!
//!pub fn main() {
//!    let mut signal = AdvancedSignal::<i32, ()>::new();
//!
//!    let state = State { a: 0 };
//!
//!    signal.connect(&method, vec![Box::new(state)]);
//!
//!    signal.call(1);
//!    signal.call(2);
//!    signal.call(1);
//!}
//!
//!advanced_method! {
//!    pub fn method(a: i32|state: State){
//!        state.a += a;
//!
//!        println!("A: {}", state.a);
//!    }
//!}
//!```

pub mod signal;

#[cfg_attr(feature = "advanced", macro_use)]
#[cfg(feature = "advanced")]
pub mod advanced_signal;
#[cfg_attr(feature = "advanced", macro_use)]
#[cfg(feature = "advanced")]
pub mod get;

pub mod prelude {
    pub use crate::signal::Signal;

    #[cfg(feature = "advanced")]
    pub use {
        crate::{advanced_method, advanced_signal::*, get::*, impl_get_ptr},
        std::any::Any,
        std::sync::{Arc, Mutex},
    };
}
