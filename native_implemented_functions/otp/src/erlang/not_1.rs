#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::term::prelude::Term;

use crate::runtime::context::*;

/// `not/1` prefix operator.
#[native_implemented::function(not/1)]
pub fn result(boolean: Term) -> exception::Result<Term> {
    let boolean_bool: bool = term_try_into_bool("boolean", boolean)?;
    let output = !boolean_bool;

    Ok(output.into())
}
