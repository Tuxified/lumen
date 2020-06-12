#[cfg(all(not(any(target_arch = "wasm32", feature = "runtime_minimal")), test))]
mod test;

use std::convert::TryInto;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::runtime::time::{system, Unit};

#[native_implemented::function(system_time/1)]
pub fn result(process: &Process, unit: Term) -> exception::Result<Term> {
    let unit_unit: Unit = unit.try_into()?;
    let big_int = system::time(unit_unit);
    let term = process.integer(big_int)?;

    Ok(term)
}
