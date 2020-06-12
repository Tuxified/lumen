#[cfg(all(not(feature = "runtime_minimal"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::runtime::registry;

#[native_implemented::function(registered/0)]
pub fn result(process: &Process) -> exception::Result<Term> {
    registry::names(process)
}
