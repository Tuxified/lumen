#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::term::prelude::Term;

/// `and/2` infix operator.
///
/// **NOTE: NOT SHORT-CIRCUITING!**  Use `andalso/2` for short-circuiting, but it doesn't enforce
/// that `right` is boolean.
#[native_implemented::function(erlang:and/2)]
fn result(left_boolean: Term, right_boolean: Term) -> exception::Result<Term> {
    boolean_infix_operator!(left_boolean, right_boolean, &)
}
