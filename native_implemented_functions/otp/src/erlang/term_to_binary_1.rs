#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::erlang::term_to_binary::term_to_binary;

#[native_implemented::function(term_to_binary/1)]
pub fn result(process: &Process, term: Term) -> exception::Result<Term> {
    term_to_binary(process, term, Default::default())
}
