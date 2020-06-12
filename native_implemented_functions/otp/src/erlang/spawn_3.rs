#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::erlang::spawn_apply_3;

#[native_implemented::function(spawn/3)]
pub fn result(
    process: &Process,
    module: Term,
    function: Term,
    arguments: Term,
) -> exception::Result<Term> {
    spawn_apply_3::result(process, Default::default(), module, function, arguments)
}
