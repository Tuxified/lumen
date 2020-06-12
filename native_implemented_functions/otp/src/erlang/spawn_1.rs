#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::erlang::spawn_apply_1;

#[native_implemented::function(spawn/1)]
pub fn result(process: &Process, function: Term) -> exception::Result<Term> {
    spawn_apply_1::result(process, Default::default(), function)
}
