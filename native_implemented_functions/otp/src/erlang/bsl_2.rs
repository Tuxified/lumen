#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

/// `bsl/2` infix operator.
#[native_implemented::function(bsl/2)]
pub fn result(process: &Process, integer: Term, shift: Term) -> exception::Result<Term> {
    bitshift_infix_operator!(integer, shift, process, <<, >>)
}
