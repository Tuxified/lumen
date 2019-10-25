// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;(

use lumen_runtime_macros::native_implemented_function;

use crate::otp::erlang::cancel_timer;

#[native_implemented_function(cancel_timer/1)]
pub fn native(process: &Process, timer_reference: Term) -> exception::Result {
    cancel_timer(timer_reference, Default::default(), process)
}
