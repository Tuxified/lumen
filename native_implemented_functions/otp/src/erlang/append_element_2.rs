#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::function(append_element/2)]
fn result(process: &Process, tuple: Term, element: Term) -> exception::Result<Term> {
    let internal = term_try_into_tuple!(tuple)?;
    let new_tuple = process.tuple_from_slices(&[&internal[..], &[element]])?;

    Ok(new_tuple)
}
