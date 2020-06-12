#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

/// `-/2` infix operator
#[native_implemented::function(-/2)]
pub fn result(process: &Process, minuend: Term, subtrahend: Term) -> exception::Result<Term> {
    number_infix_operator!(minuend, subtrahend, process, checked_sub, -)
}
