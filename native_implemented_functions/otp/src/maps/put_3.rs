#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

#[native_implemented::function(put/3)]
pub fn result(process: &Process, key: Term, value: Term, map: Term) -> exception::Result<Term> {
    let boxed_map = term_try_into_map_or_badmap!(process, map)?;

    match boxed_map.put(key, value) {
        Some(hash_map) => Ok(process.map_from_hash_map(hash_map)?),
        None => Ok(map),
    }
}
